use super::{ClientOp, HeaderMap, ServerOp, StatusCode, Subject};
use crate::{Config, Error};
use async_nats::header::{IntoHeaderName, IntoHeaderValue};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use memchr::memmem;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    cmp,
    fmt::{self, Write},
    io,
    str::FromStr,
};
use tokio_util::codec::{AnyDelimiterCodecError, Decoder, Encoder};

pub const DELIMITER: &str = "\r\n";
pub const VERSION: &str = "NATS/1.0";

/// The core codec for encoding and decoding messages.
///
/// See [`Connection`] methods `try_read_op` and `enqueue_write_op` for
/// original [`async_nats`] implementations.
///
/// [`Connection`]: async_nats::Connection
#[derive(Clone, Debug)]
pub struct Codec<T = ClientOp> {
    max_payload_len: usize,
    inner: chunk_codec::ChunkCodec,
    partial_op: Option<PartialOp<T>>,
}

impl Codec<ClientOp> {
    fn is_pub(&self) -> bool {
        matches!(
            self.partial_op.as_ref().map(|partial| &partial.op),
            Some(ClientOp::Publish { .. })
        )
    }
}

impl Codec<ServerOp> {
    fn is_msg(&self) -> bool {
        matches!(
            self.partial_op.as_ref().map(|partial| &partial.op),
            Some(ServerOp::Message { .. })
        )
    }
}

impl<T> Codec<T> {
    fn partial_decode(
        &mut self,
        partial_op: PartialOp<T>,
        buf: &mut BytesMut,
    ) -> Result<Option<<Self as Decoder>::Item>, Error>
    where
        Self: Decoder<Error = Error>,
    {
        let len = partial_op.total_len();
        buf.reserve(len);
        self.inner.set_min_length(len);
        self.partial_op = Some(partial_op);
        self.decode(buf)
    }

    fn finish_decode(&mut self) -> PartialOp<T> {
        self.inner.reset_min_length();
        self.partial_op.take().unwrap()
    }
}

impl<T> Default for Codec<T> {
    fn default() -> Self {
        Self {
            max_payload_len: Config::default_max_payload(),
            inner: chunk_codec::ChunkCodec::default(),
            partial_op: None,
        }
    }
}

#[derive(Clone, Debug)]
struct PartialOp<T> {
    op: T,
    headers_len: Option<usize>,
    payload_len: usize,
}

impl<T> PartialOp<T> {
    fn total_len(&self) -> usize {
        self.headers_len.unwrap_or(0) + self.payload_len
    }
}

impl Decoder for Codec<ClientOp> {
    type Item = ClientOp;
    type Error = Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut chunk = match self.inner.decode(buf)? {
            None => return Ok(None),
            Some(chunk) => chunk,
        };

        if chunk.starts_with(b"CONNECT") {
            let info = util::read_info("CONNECT", &mut chunk)?;
            return Ok(Some(ClientOp::Connect(info)));
        }
        if chunk.starts_with(b"INFO") {
            let info = util::read_info("INFO", &mut chunk)?;
            return Ok(Some(ClientOp::Info(info)));
        }
        if chunk.starts_with(b"PING") {
            return Ok(Some(ClientOp::Ping));
        }
        if chunk.starts_with(b"PONG") {
            return Ok(Some(ClientOp::Pong));
        }

        // sub, unsub
        if chunk.starts_with(b"SUB") {
            let (subject, queue_group, sid) = util::read_sub_header("SUB", &mut chunk)?;

            return Ok(Some(ClientOp::Subscribe {
                sid,
                subject,
                queue_group,
            }));
        }
        if chunk.starts_with(b"UNSUB") {
            let (sid, max_msgs) = util::read_unsub_header("UNSUB", &mut chunk)?;

            return Ok(Some(ClientOp::Unsubscribe { sid, max_msgs }));
        }

        // pub + hpub
        // first pass: parse until first \r\n, setting min_length for next pass
        if chunk.starts_with(b"PUB") {
            let partial_op = util::read_partial_pub("PUB", &mut chunk)?;
            return self.partial_decode(partial_op, buf);
        }
        if chunk.starts_with(b"HPUB") {
            let partial_op = util::read_partial_pub("HPUB", &mut chunk)?;
            return self.partial_decode(partial_op, buf);
        }
        // next pass: parse chunk w/ headers + payload
        if self.is_pub() {
            let PartialOp {
                headers_len,
                payload_len,
                op:
                    ClientOp::Publish {
                        subject,
                        reply: reply_to,
                        ..
                    },
            } = self.finish_decode()
            else {
                unreachable!();
            };

            // try parse headers

            // try parse payload

            let (headers, payload) = if let Some(headers_len) = headers_len {
                // hpub
                let headers_bytes = chunk.split_to(headers_len);
                let (_, _, headers) = util::read_header_map(&headers_bytes)?;

                (Some(headers), chunk)
            } else {
                (None, chunk)
            };

            debug_assert!(payload_len == payload.len());
            return Ok(Some(ClientOp::Publish {
                subject,
                reply: reply_to,
                headers,
                payload,
            }));
        }

        Err(Error::codec(anyhow::anyhow!("invalid server op")))
    }

    // fn decode_eof(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
    //     match self.decode(buf)? {
    //         Some(frame) => Ok(Some(frame)),
    //         None => {
    //             if buf.is_empty() {
    //                 Ok(None)
    //             } else {
    //                 Err(io::Error::new(io::ErrorKind::Other, "bytes remaining on stream").into())
    //             }
    //         }
    //     }
    // }
}

impl Decoder for Codec<ServerOp> {
    type Item = ServerOp;
    type Error = Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut chunk = match self.inner.decode(buf)? {
            None => return Ok(None),
            Some(chunk) => chunk,
        };

        if chunk.starts_with(b"CONNECT") {
            let info = util::read_info("CONNECT", &mut chunk)?.ok_or_else(|| {
                Error::codec(anyhow::anyhow!("missing/erroneous CONNECT message"))
            })?;
            return Ok(Some(ServerOp::Connect(info)));
        }
        if chunk.starts_with(b"INFO") {
            let info = util::read_info("INFO", &mut chunk)?
                .ok_or_else(|| Error::codec(anyhow::anyhow!("missing/erroneous INFO message")))?;
            return Ok(Some(ServerOp::Info(info)));
        }
        if chunk.starts_with(b"PING") {
            return Ok(Some(ServerOp::Ping));
        }
        if chunk.starts_with(b"PONG") {
            return Ok(Some(ServerOp::Pong));
        }

        // rs+, rs-
        if chunk.starts_with(b"RS+") {
            let (account, subject, queue_group) = util::read_rsub_header("RS+", true, &mut chunk)?;

            return Ok(Some(ServerOp::Subscribe {
                account,
                subject,
                queue_group,
            }));
        }
        if chunk.starts_with(b"RS-") {
            let (account, subject, _) = util::read_rsub_header("RS-", false, &mut chunk)?;

            return Ok(Some(ServerOp::Unsubscribe { account, subject }));
        }

        // msg, hmsg, rmsg
        if chunk.starts_with(b"MSG") {
            let partial_op = util::read_partial_msg("MSG", &mut chunk)?;
            return self.partial_decode(partial_op, buf);
        }
        if chunk.starts_with(b"HMSG") {
            let partial_op = util::read_partial_msg("HMSG", &mut chunk)?;
            return self.partial_decode(partial_op, buf);
        }
        if chunk.starts_with(b"RMSG") {
            let partial_op = util::read_partial_msg("RMSG", &mut chunk)?;
            return self.partial_decode(partial_op, buf);
        }
        if self.is_msg() {
            let PartialOp {
                headers_len,
                payload_len,
                op:
                    ServerOp::Message {
                        account,
                        sid,
                        subject,
                        reply_to,
                        ..
                    },
            } = self.finish_decode()
            else {
                unreachable!();
            };

            // TODO: try parse headers (potentially across packets), then payload
            let (status, description, headers, payload) = if let Some(headers_len) = headers_len {
                // hmsg
                let headers_bytes = chunk.split_to(headers_len + (2 * DELIMITER.len()));
                let (status, description, headers) = util::read_header_map(&headers_bytes)?;

                (status, description, Some(headers), chunk)
            } else {
                (None, None, None, chunk)
            };

            debug_assert!(payload_len == payload.len());
            return Ok(Some(ServerOp::Message {
                account,
                sid,
                subject,
                reply_to,
                headers,
                status,
                description,
                payload,
            }));
        }

        Err(Error::codec(anyhow::anyhow!("unknown server op")))
    }
}

impl<T> Encoder<ClientOp> for Codec<T> {
    type Error = Error;

    fn encode(&mut self, item: ClientOp, buf: &mut BytesMut) -> Result<(), Self::Error> {
        let reserve = item.encode_size_hint();
        match item {
            ClientOp::Ping => self.inner.encode("PING", buf)?,
            ClientOp::Pong => self.inner.encode("PONG", buf)?,
            ClientOp::Connect(ref info) => util::encode_info("CONNECT", info, buf)?,
            ClientOp::Info(ref info) => util::encode_info("INFO", info, buf)?,

            ClientOp::Subscribe {
                subject,
                queue_group,
                sid,
            } => match queue_group {
                None => self
                    .inner
                    .encode(format_args!("SUB {subject} {sid}"), buf)?,
                Some(queue_group) => self
                    .inner
                    .encode(format_args!("SUB {subject} {queue_group} {sid}"), buf)?,
            },
            ClientOp::Unsubscribe { sid, max_msgs } => match max_msgs {
                None => self.inner.encode(format_args!("UNSUB {sid}"), buf)?,
                Some(max_msgs) => self
                    .inner
                    .encode(format_args!("UNSUB {sid} {max_msgs}"), buf)?,
            },
            // pub
            ClientOp::Publish {
                subject,
                reply: reply_to,
                headers: None,
                payload,
            } => {
                buf.reserve(reserve);
                let reply_to = reply_to
                    .as_ref()
                    .map_or("".to_string(), |r| format!("{r} "));

                // write first chunk
                self.inner.encode(
                    format_args!("PUB {subject} {reply_to}{}", payload.len()),
                    buf,
                )?;
                self.inner.encode(payload.as_ref(), buf)?;
            }
            // hpub
            ClientOp::Publish {
                subject,
                reply: reply_to,
                headers: Some(header_map),
                payload,
            } => {
                buf.reserve(reserve);
                let reply_to = reply_to
                    .as_ref()
                    .map_or("".to_string(), |r| format!("{r} "));

                // write first chunk
                let headers_len = util::header_map_len(None, None, &header_map);

                self.inner.encode(
                    format_args!(
                        "HPUB {subject} {reply_to}{headers_len} {}",
                        headers_len + payload.len()
                    ),
                    buf,
                )?;

                // write remaining chunk(s)
                util::encode_header_map(None, None, &header_map, buf)?;
                self.inner.encode(payload.as_ref(), buf)?;
            }
        };

        Ok(())
    }
}

impl<T> Encoder<ServerOp> for Codec<T> {
    type Error = Error;

    fn encode(&mut self, item: ServerOp, buf: &mut BytesMut) -> Result<(), Self::Error> {
        let reserve = item.encode_size_hint();
        match item {
            ServerOp::Ok => self.inner.encode("+OK", buf)?,
            ServerOp::Err(ref err) => self.inner.encode(format_args!("-ERR {err}"), buf)?,
            ServerOp::Ping => self.inner.encode("PING", buf)?,
            ServerOp::Pong => self.inner.encode("PONG", buf)?,
            ServerOp::Connect(ref info) => util::encode_info("CONNECT", info, buf)?,
            ServerOp::Info(ref info) => util::encode_info("INFO", info, buf)?,

            ServerOp::Subscribe {
                account,
                subject,
                queue_group,
            } => match queue_group {
                None => self
                    .inner
                    .encode(format_args!("RS+ {account} {subject}"), buf)?,
                Some((queue_group, None)) => self
                    .inner
                    .encode(format_args!("RS+ {account} {subject} {queue_group}"), buf)?,
                Some((queue_group, Some(weight))) => self.inner.encode(
                    format_args!("RS+ {account} {subject} {queue_group} {weight}"),
                    buf,
                )?,
            },
            ServerOp::Unsubscribe { account, subject } => {
                self.inner
                    .encode(format_args!("RS- {account} {subject}"), buf)?;
            }
            ServerOp::Message {
                sid,
                account,
                subject,
                reply_to,
                headers,
                status,
                description,
                payload,
            } => {
                buf.reserve(reserve);

                let reply_to = reply_to
                    .as_ref()
                    .map_or("".to_string(), |r| format!("{r} "));
                let description = description.as_deref();

                // write first chunk
                match (&account, &headers) {
                    // msg
                    (None, None) => {
                        self.inner.encode(
                            format_args!("MSG {subject} {sid} {reply_to}{}", payload.len()),
                            buf,
                        )?;
                    }
                    // hmsg
                    (None, Some(header_map)) => {
                        let headers_len = util::header_map_len(status, description, header_map);

                        self.inner.encode(
                            format_args!(
                                "HMSG {subject} {sid} {reply_to}{headers_len} {}",
                                headers_len + payload.len()
                            ),
                            buf,
                        )?;
                    }
                    // rmsg
                    (Some(account), _) => {
                        self.inner.encode(
                            format_args!("RMSG {account} {subject} {reply_to}{}", payload.len()),
                            buf,
                        )?;
                    }
                };

                // write remaining chunk(s)
                match (account, headers) {
                    (Some(_), _) | (_, None) => self.inner.encode(payload.as_ref(), buf)?,
                    (_, Some(header_map)) => {
                        util::encode_header_map(status, description, &header_map, buf)?;
                        self.inner.encode(payload.as_ref(), buf)?;
                    }
                }
            }
        };

        Ok(())
    }
}

impl<T> Encoder<Option<ServerOp>> for Codec<T> {
    type Error = Error;

    fn encode(&mut self, item: Option<ServerOp>, buf: &mut BytesMut) -> Result<(), Self::Error> {
        match item {
            Some(item) => self.encode(item, buf),
            None => Ok(()),
        }
    }
}

impl ClientOp {
    fn encode_size_hint(&self) -> usize {
        256
    }
}

impl ServerOp {
    fn encode_size_hint(&self) -> usize {
        match self {
            Self::Message {
                account,
                subject,
                headers,
                status,
                description,
                payload,
                ..
            } => {
                5 + (subject.len() + 1)
                    + account.as_ref().map_or(0, |a| a.len() + 1)
                    + headers.as_ref().map_or(0, |h| {
                        util::header_map_len(*status, description.as_ref().map(|s| s.as_str()), h)
                    })
                    + payload.len()
            }
            _ => 256,
        }
    }
}

mod util {
    use super::*;

    // encoder helpers

    pub fn encode_info<I: Serialize>(verb: &str, info: I, buf: &mut BytesMut) -> Result<(), Error> {
        buf.reserve(256);
        buf.put_slice(verb.as_bytes());
        buf.put_slice(b" ");
        {
            let mut vec = Vec::with_capacity(256);
            serde_json::to_writer(&mut vec, &info).map_err(Error::codec)?;
            buf.extend_from_slice(&vec);
        };
        buf.put_slice(DELIMITER.as_bytes());
        Ok(())
    }

    pub fn header_map_len(
        status: Option<StatusCode>,
        description: Option<&str>,
        header_map: &HeaderMap,
    ) -> usize {
        let mut len = VERSION.len()
            + status.map_or(0, |_| 4)
            + description.map_or(0, |s| 1 + s.len())
            + DELIMITER.len();
        for (name, value) in header_map.iter() {
            for val in value.iter() {
                len += AsRef::<[u8]>::as_ref(name).len()
                    + 2
                    + AsRef::<[u8]>::as_ref(val).len()
                    + DELIMITER.len();
            }
        }
        len + DELIMITER.len()
    }

    pub fn encode_header_map(
        status: Option<StatusCode>,
        description: Option<&str>,
        header_map: &HeaderMap,
        buf: &mut BytesMut,
    ) -> Result<(), Error> {
        buf.put_slice(VERSION.as_bytes());
        if let Some(status) = status {
            write!(buf, " {}", status).map_err(Error::codec)?;
        }
        if let Some(description) = description {
            write!(buf, " {}", description).map_err(Error::codec)?;
        }
        buf.put_slice(DELIMITER.as_bytes());

        for (name, value) in header_map.iter() {
            for val in value.iter() {
                buf.put_slice(name.as_ref());
                buf.put_slice(b": ");
                buf.put_slice(val.as_ref());
                buf.put_slice(DELIMITER.as_bytes());
            }
        }
        buf.put_slice(DELIMITER.as_bytes());
        Ok(())
    }

    // decoder helplers

    pub fn read_info<T: DeserializeOwned>(verb: &str, buf: &mut Bytes) -> Result<Option<T>, Error> {
        buf.advance(verb.len() + 1);
        Ok(serde_json::from_slice(buf.as_ref()).ok())
    }

    pub fn read_header_map(
        buf: &[u8],
    ) -> Result<(Option<StatusCode>, Option<String>, HeaderMap), Error> {
        let mut headers = HeaderMap::new();
        let mut lines = core::str::from_utf8(buf)
            .map_err(Error::codec)?
            .lines()
            .peekable();

        let (status, description) = {
            let version_line = lines
                .next()
                .ok_or_else(|| Error::codec(anyhow::anyhow!("missing version line")))?;
            let version_line_suffix = version_line
                .strip_prefix(VERSION)
                .map(str::trim)
                .ok_or_else(|| {
                    Error::codec(anyhow::anyhow!(
                        "header version line does not begin with `{VERSION}`",
                    ))
                })?;

            let (status, description) = version_line_suffix
                .split_once(' ')
                .map(|(status, description)| (status.trim(), description.trim()))
                .unwrap_or((version_line_suffix, ""));

            let status = if !status.is_empty() {
                Some(status.parse().map_err(Error::codec)?)
            } else {
                None
            };
            let description = if !description.is_empty() {
                Some(description.to_owned())
            } else {
                None
            };
            (status, description)
        };

        while let Some(line) = lines.next() {
            if line.is_empty() {
                continue;
            }
            let (name, val) = line
                .split_once(':')
                .ok_or_else(|| Error::codec(anyhow::anyhow!("invalid header line: {}", line)))?;

            // Read the header value, which might have been split into multiple lines
            // `trim_start` and `trim_end` do the same job as doing `value.trim().to_owned()` at the end, but without a reallocation
            let mut value = val.trim_start().to_owned();
            while let Some(v) = lines.next_if(|l| l.starts_with(char::is_whitespace)) {
                value.push_str(v);
            }

            value.truncate(value.trim_end().len());
            headers.append(name.into_header_name(), value.into_header_value());
        }

        Ok((status, description, headers))
    }

    pub fn read_partial_pub(verb: &str, buf: &mut Bytes) -> Result<PartialOp<ClientOp>, Error> {
        buf.advance(verb.len());

        let chunk_str = core::str::from_utf8(buf).map_err(Error::codec)?;
        let num_parts = chunk_str
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .count();

        let mut parts = chunk_str.split_whitespace().filter(|s| !s.is_empty());

        let subject = parts
            .next()
            .map(Subject::from)
            .ok_or_else(|| Error::codec(anyhow::anyhow!("{} missing subject", verb)))?;

        let res = match (verb, num_parts) {
            // pub
            ("PUB", 2 | 3) => {
                let reply_to = (num_parts == 3)
                    .then(|| parts.next())
                    .flatten()
                    .map(Subject::from);
                let payload_len =
                    util::from_str_part(parts.next(), "missing/erroneous payload len")?;

                let op = ClientOp::Publish {
                    subject,
                    reply: reply_to,
                    headers: None,
                    payload: Bytes::new(),
                };
                Ok(super::PartialOp {
                    headers_len: None,
                    payload_len,
                    op,
                })
            }
            // hpub
            ("HPUB", 3 | 4) => {
                let reply_to = (num_parts == 4)
                    .then(|| parts.next())
                    .flatten()
                    .map(Subject::from);
                let headers_len =
                    util::from_str_part(parts.next(), "missing/erroneous headers len")?;
                let total_len: usize =
                    util::from_str_part(parts.next(), "missing/erroneous total len")?;

                let op = ClientOp::Publish {
                    subject,
                    reply: reply_to,
                    headers: None,
                    payload: Bytes::new(),
                };
                Ok(super::PartialOp {
                    headers_len: Some(headers_len),
                    payload_len: total_len - headers_len,
                    op,
                })
            }
            _ => Err(Error::codec(anyhow::anyhow!("invalid {} message", verb))),
        };

        buf.advance(chunk_str.len());
        res
    }

    pub fn read_sub_header(
        verb: &str,
        buf: &mut Bytes,
    ) -> Result<(Subject, Option<String>, u64), Error> {
        buf.advance(verb.len());

        let chunk_str = core::str::from_utf8(buf).map_err(Error::codec)?;

        let num_parts = chunk_str
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .count();
        let mut parts = chunk_str.split_whitespace().filter(|s| !s.is_empty());

        let subject = parts.next().map(Subject::from).ok_or_else(|| {
            Error::codec(anyhow::anyhow!("{} missing/invalid account/subject", verb))
        })?;

        fn get_sid(part: Option<&str>) -> Result<u64, Error> {
            part.and_then(|t| u64::from_str(t).ok())
                .ok_or_else(|| Error::codec(anyhow::anyhow!("invalid sid")))
        }

        let res = match num_parts {
            2 => {
                let sid = get_sid(parts.next())?;
                (subject, None, sid)
            }
            3 => {
                let queue_group = parts.next().map(|s| s.to_string());
                let sid = get_sid(parts.next())?;
                (subject, queue_group, sid)
            }
            _ => return Err(Error::codec(anyhow::anyhow!("invalid {} message", verb))),
        };

        buf.advance(chunk_str.len());
        Ok(res)
    }

    pub fn read_rsub_header<T, U>(
        verb: &str,
        try_queue_group: bool,
        buf: &mut Bytes,
    ) -> Result<(T, U, Option<(String, Option<u32>)>), Error>
    where
        T: FromStr,
        U: FromStr,
    {
        buf.advance(verb.len());

        let chunk_str = core::str::from_utf8(buf).map_err(Error::codec)?;

        let mut parts = chunk_str.split_whitespace().filter(|s| !s.is_empty());

        let t = parts
            .next()
            .and_then(|t| T::from_str(t).ok())
            .ok_or_else(|| {
                Error::codec(anyhow::anyhow!("{} missing/invalid account/subject", verb))
            })?;
        let u = parts
            .next()
            .and_then(|u| U::from_str(u).ok())
            .ok_or_else(|| Error::codec(anyhow::anyhow!("{} missing/invalid subject/sid", verb)))?;

        let res = match try_queue_group.then(|| parts.next()).flatten() {
            None => (t, u, None),
            Some(queue_group) => {
                let queue_group = queue_group.to_string();
                let weight = parts.next().and_then(|s| s.parse().ok());
                (t, u, Some((queue_group, weight)))
            }
        };

        buf.advance(chunk_str.len());
        Ok(res)
    }

    pub fn read_unsub_header(verb: &str, buf: &mut Bytes) -> Result<(u64, Option<u64>), Error> {
        buf.advance(verb.len());

        let chunk_str = core::str::from_utf8(buf).map_err(Error::codec)?;

        let mut parts = chunk_str.split_whitespace().filter(|s| !s.is_empty());
        let sid = parts
            .next()
            .and_then(|t| u64::from_str(t).ok())
            .ok_or_else(|| {
                Error::codec(anyhow::anyhow!("{} missing/invalid account/subject", verb))
            })?;
        let max_msgs = parts.next().and_then(|u| u64::from_str(u).ok());

        Ok((sid, max_msgs))
    }

    pub fn read_partial_msg(
        verb: &str,
        buf: &mut Bytes,
    ) -> Result<super::PartialOp<ServerOp>, Error> {
        buf.advance(verb.len());

        let chunk_str = core::str::from_utf8(buf).map_err(Error::codec)?;
        let num_parts = chunk_str
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .count();

        let mut parts = chunk_str.split_whitespace().filter(|s| !s.is_empty());
        let first = parts
            .next()
            .ok_or_else(|| Error::codec(anyhow::anyhow!("{} missing subject", verb)))?;

        let res = match (verb, num_parts) {
            ("MSG", 3 | 4) => {
                let sid = from_str_part(parts.next(), "missing/erroneous sid")?;
                let reply_to = (num_parts == 4)
                    .then(|| parts.next().map(Subject::from))
                    .flatten();
                let payload_len = from_str_part(parts.next(), "missing/erroneous payload len")?;

                let op = ServerOp::Message {
                    sid,
                    account: None,
                    subject: Subject::from(first),
                    reply_to,
                    headers: None,
                    status: None,
                    description: None,
                    payload: Bytes::new(),
                };
                Ok(super::PartialOp {
                    headers_len: None,
                    payload_len,
                    op,
                })
            }
            ("RMSG", 3 | 4) => {
                let subject = parts
                    .next()
                    .ok_or_else(|| Error::codec(anyhow::anyhow!("{} missing subject", verb)))?;
                let reply_to = (num_parts == 4)
                    .then(|| parts.next().map(Subject::from))
                    .flatten();
                let payload_len = from_str_part(parts.next(), "missing/erroneous payload len")?;

                let op = ServerOp::Message {
                    sid: 0,
                    account: Some(first.into()),
                    subject: subject.into(),
                    reply_to,
                    headers: None,
                    status: None,
                    description: None,
                    payload: Bytes::new(),
                };
                Ok(super::PartialOp {
                    headers_len: None,
                    payload_len,
                    op,
                })
            }
            // pub, with reply_to
            ("HMSG", 4 | 5) => {
                let sid = from_str_part(parts.next(), "missing/erroneous sid")?;
                let reply_to = (num_parts == 5)
                    .then(|| parts.next().map(Subject::from))
                    .flatten();
                let headers_len = from_str_part(parts.next(), "missing/erroneous headers len")?;
                let total_len: usize = from_str_part(parts.next(), "missing/erroneous total len")?;

                let op = ServerOp::Message {
                    sid,
                    account: None,
                    subject: Subject::from(first),
                    reply_to,
                    headers: None,
                    status: None,
                    description: None,
                    payload: Bytes::new(),
                };
                Ok(super::PartialOp {
                    headers_len: Some(headers_len),
                    payload_len: total_len - headers_len,
                    op,
                })
            }
            _ => Err(Error::codec(anyhow::anyhow!("invalid {} message", verb))),
        };

        buf.advance(chunk_str.len());
        res
    }

    fn from_str_part<T: FromStr>(part: Option<&str>, err_msg: &'static str) -> Result<T, Error> {
        part.and_then(|s| s.parse::<T>().ok())
            .ok_or_else(|| Error::codec(anyhow::anyhow!(err_msg)))
    }
}

mod chunk_codec {
    use super::*;

    /// Utility codec for decoding chunks ending in `\r\n`.
    /// Adapted from [`tokio_util::codec::AnyDelimiterCodec`].
    #[derive(Clone, Debug)]
    pub struct ChunkCodec {
        // Stored index of the next index to examine for the delimiter character.
        // This is used to optimize searching.
        // For example, if `decode` was called with `abc` and the delimiter is '{}', it would hold `3`,
        // because that is the next index to examine.
        // The next time `decode` is called with `abcde}`, the method will
        // only look at `de}` before returning.
        next_index: usize,

        /// The maximum length for a given chunk. If `usize::MAX`, chunks will be
        /// read until a delimiter character is reached.
        max_length: usize,
        min_length: usize,

        /// Are we currently discarding the remainder of a chunk which was over
        /// the length limit?
        is_discarding: bool,
    }

    impl ChunkCodec {
        pub fn set_min_length(&mut self, min_length: usize) {
            self.min_length = min_length;
        }
        pub fn reset_min_length(&mut self) {
            self.min_length = 0;
        }
        pub fn set_max_length(&mut self, max_length: usize) {
            self.max_length = max_length;
        }
        pub fn reset_max_length(&mut self) {
            self.max_length = usize::MAX;
        }

        pub fn with_max_length(mut self, max_length: usize) -> Self {
            self.max_length = max_length;
            self
        }
    }

    impl Default for ChunkCodec {
        fn default() -> Self {
            Self {
                next_index: 0,
                min_length: 0,
                max_length: usize::MAX,
                is_discarding: false,
            }
        }
    }

    impl Decoder for ChunkCodec {
        type Item = Bytes;
        type Error = Error;

        fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
            debug_assert!(self.min_length < self.max_length);
            loop {
                // Determine where we'll start looking for a delimiter.
                let start = cmp::max(self.next_index, self.min_length);

                // Determine how far into the buffer we'll search for a delimiter. If
                // there's no max_length set, we'll read to the end of the buffer.
                let read_to = cmp::min(self.max_length.saturating_add(1), buf.len());

                if start >= read_to {
                    self.next_index = read_to;
                    return Ok(None);
                }

                let new_chunk_offset = memmem::find(&buf[start..read_to], DELIMITER.as_bytes());

                match (self.is_discarding, new_chunk_offset) {
                    (false, Some(offset)) => {
                        // Found a chunk!
                        self.next_index = 0;

                        let new_chunk_index = offset + start;
                        let mut chunk = buf.split_to(new_chunk_index + DELIMITER.len());
                        chunk.truncate(chunk.len() - DELIMITER.len());
                        let chunk = chunk.freeze();
                        return Ok(Some(chunk));
                    }
                    (false, None) if buf.len() > self.max_length => {
                        // Reached the maximum length without finding a
                        // new chunk, return an error and start discarding on the
                        // next call.
                        self.is_discarding = true;
                        return Err(Error::codec(AnyDelimiterCodecError::MaxChunkLengthExceeded))?;
                    }
                    (false, None) => {
                        // We didn't find a chunk or reach the length limit, so the next
                        // call will resume searching at the current offset.
                        self.next_index = read_to;
                        return Ok(None);
                    }
                    (true, Some(offset)) => {
                        // If we found a new chunk, discard up to that offset and
                        // then stop discarding. On the next iteration, we'll try
                        // to read a chunk normally.
                        buf.advance(offset + self.next_index + DELIMITER.len());
                        self.is_discarding = false;
                        self.next_index = 0;
                    }
                    (true, None) => {
                        // Otherwise, we didn't find a new chunk, so we'll discard
                        // everything we read. On the next iteration, we'll continue
                        // discarding up to max_len bytes unless we find a new chunk.
                        buf.advance(read_to);
                        self.next_index = 0;
                        if buf.is_empty() {
                            return Ok(None);
                        }
                    }
                }
            }
        }

        fn decode_eof(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
            Ok(match self.decode(buf)? {
                Some(frame) => Some(frame),
                None => {
                    // return remaining data, if any
                    if buf.is_empty() {
                        None
                    } else {
                        let chunk = buf.split_to(buf.len());
                        self.next_index = 0;
                        Some(chunk.freeze())
                    }
                }
            })
        }
    }

    impl Encoder<&[u8]> for ChunkCodec {
        type Error = io::Error;

        fn encode(&mut self, item: &[u8], buf: &mut BytesMut) -> Result<(), Self::Error> {
            buf.reserve(item.len() + DELIMITER.len());
            buf.put(item);
            buf.put_slice(DELIMITER.as_bytes());
            Ok(())
        }
    }

    impl<const N: usize> Encoder<[u8; N]> for ChunkCodec {
        type Error = io::Error;

        fn encode(&mut self, item: [u8; N], buf: &mut BytesMut) -> Result<(), Self::Error> {
            self.encode(&item[..], buf)
        }
    }

    impl Encoder<Bytes> for ChunkCodec {
        type Error = io::Error;

        fn encode(&mut self, item: Bytes, buf: &mut BytesMut) -> Result<(), Self::Error> {
            buf.reserve(item.len() + DELIMITER.len());
            buf.put(item);
            buf.put_slice(DELIMITER.as_bytes());
            Ok(())
        }
    }

    impl Encoder<&str> for ChunkCodec {
        type Error = io::Error;

        fn encode(&mut self, item: &str, buf: &mut BytesMut) -> Result<(), Self::Error> {
            self.encode(item.as_bytes(), buf)
        }
    }

    impl<'a> Encoder<fmt::Arguments<'a>> for ChunkCodec {
        type Error = io::Error;

        fn encode(
            &mut self,
            item: fmt::Arguments<'a>,
            buf: &mut BytesMut,
        ) -> Result<(), Self::Error> {
            write!(buf, "{}{}", item, DELIMITER).map_err(|_| io::Error::other("fmt error"))?;
            Ok(())
        }
    }

    #[test]
    fn test_decode() -> io::Result<()> {
        let mut codec = ChunkCodec::default();
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"hello\r\nworld\r\n");

        let chunk = codec.decode(&mut buf)?.unwrap();
        assert_eq!(&chunk, b"hello".as_ref());
        let chunk = codec.decode(&mut buf)?.unwrap();
        assert_eq!(&chunk, b"world".as_ref());

        // with_min_length should concat past delimiters
        let mut codec = ChunkCodec::default();
        codec.set_min_length(10);
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"hello\r\nworld\r\n");

        let chunk = codec.decode(&mut buf)?.unwrap();
        assert_eq!(&chunk, b"hello\r\nworld".as_ref());

        // min_length error
        codec = ChunkCodec::default();
        codec.set_min_length(15);
        buf.extend_from_slice(b"hello\r\nworld\r\n");

        let chunk = codec.decode(&mut buf)?;
        assert_eq!(chunk, None);

        Ok(())
    }

    #[test]
    fn test_encode() -> io::Result<()> {
        let mut codec = ChunkCodec::default();
        let mut buf = BytesMut::new();

        codec.encode(b"hello".as_ref(), &mut buf)?;
        codec.encode(b"world".as_ref(), &mut buf)?;
        assert_eq!(&buf, b"hello\r\nworld\r\n".as_ref());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let mut codec = Codec::<ClientOp>::default();

        let mut buf = BytesMut::new();
        codec.encode(ServerOp::Ping, &mut buf).unwrap();
        assert_eq!(buf, "PING\r\n");

        let mut buf = BytesMut::new();
        codec.encode(ServerOp::Pong, &mut buf).unwrap();
        assert_eq!(buf, "PONG\r\n");

        // pub
        let mut buf = BytesMut::new();
        codec
            .encode(
                ClientOp::Publish {
                    subject: Subject::from_static("FOO.BAR"),
                    reply: None,
                    headers: None,
                    payload: Bytes::from_static(b"Hello World"),
                },
                &mut buf,
            )
            .unwrap();
        assert_eq!(buf, "PUB FOO.BAR 11\r\nHello World\r\n");

        // hpub
        let mut buf = BytesMut::new();
        codec
            .encode(
                ClientOp::Publish {
                    subject: Subject::from_static("FOO.BAR"),
                    reply: Some(Subject::from_static("INBOX.67")),
                    headers: Some({
                        let mut headers = HeaderMap::new();
                        headers.insert("Header", "Y");
                        headers
                    }),
                    payload: Bytes::from_static(b"Hello World"),
                },
                &mut buf,
            )
            .unwrap();
        assert_eq!(
            buf,
            "HPUB FOO.BAR INBOX.67 23 34\r\nNATS/1.0\r\nHeader: Y\r\n\r\nHello World\r\n"
        );

        // msg
        let mut buf = BytesMut::new();
        codec
            .encode(
                ServerOp::Message {
                    sid: 42,
                    subject: Subject::from_static("FOO.BAR"),
                    payload: Bytes::from_static(b"Hello World"),
                    reply_to: None,
                    headers: None,
                    status: None,
                    description: None,
                    account: None,
                },
                &mut buf,
            )
            .unwrap();
        assert_eq!(buf, "MSG FOO.BAR 42 11\r\nHello World\r\n");

        // hmsg with just status code
        let mut buf = BytesMut::new();
        codec
            .encode(
                ServerOp::Message {
                    sid: 42,
                    subject: Subject::from_static("FOO.BAR"),
                    payload: Bytes::from_static(b"Hello World"),
                    reply_to: None,
                    headers: Some(HeaderMap::new()),
                    status: Some(StatusCode::NO_RESPONDERS),
                    description: None,
                    account: None,
                },
                &mut buf,
            )
            .unwrap();
        assert_eq!(
            buf,
            "HMSG FOO.BAR 42 16 27\r\nNATS/1.0 503\r\n\r\nHello World\r\n"
        );

        // hmsg status, description and headers
        let mut buf = BytesMut::new();
        codec
            .encode(
                ServerOp::Message {
                    sid: 42,
                    subject: Subject::from_static("FOO.BAR"),
                    payload: Bytes::from_static(b"Hello World"),
                    reply_to: None,
                    headers: Some({
                        let mut headers = HeaderMap::new();
                        headers.insert("Header", "Y");
                        headers
                    }),
                    status: Some(StatusCode::NO_RESPONDERS),
                    description: Some("no_responders".to_string()),
                    account: None,
                },
                &mut buf,
            )
            .unwrap();
        assert_eq!(
            buf,
            "HMSG FOO.BAR 42 41 52\r\nNATS/1.0 503 no_responders\r\nHeader: Y\r\n\r\nHello World\r\n"
        );

        // hmsg
    }

    #[test]
    fn test_decode_client_op() {
        let mut codec = Codec::<ClientOp>::default();

        let mut buf = BytesMut::from(&b"PING\r\n"[..]);
        let msg = codec.decode(&mut buf).unwrap().unwrap();
        assert_eq!(msg, ClientOp::Ping);

        let mut buf = BytesMut::from(&b"PONG\r\n"[..]);
        let msg = codec.decode(&mut buf).unwrap().unwrap();
        assert_eq!(msg, ClientOp::Pong);

        let mut buf = BytesMut::from(&b"CONNECT {}\r\n"[..]);
        let msg = codec.decode(&mut buf).unwrap().unwrap();
        assert_eq!(msg, ClientOp::Connect(None));

        // pub
        let mut buf = BytesMut::from(&b"PUB FOO.BAR INBOX.67 11\r\nHello World\r\n"[..]);
        let msg = codec.decode(&mut buf).unwrap().unwrap();
        assert_eq!(
            msg,
            ClientOp::Publish {
                subject: Subject::from_static("FOO.BAR"),
                reply: Some(Subject::from_static("INBOX.67")),
                headers: None,
                payload: Bytes::from_static(b"Hello World"),
            }
        );

        // hpub
        let mut buf = BytesMut::from(
            &b"HPUB FOO.BAR INBOX.67 23 34\r\nNATS/1.0\r\nHeader: Y\r\n\r\nHello World\r\n"[..],
        );
        let msg = codec.decode(&mut buf).unwrap().unwrap();
        assert_eq!(
            msg,
            ClientOp::Publish {
                subject: Subject::from_static("FOO.BAR"),
                reply: Some(Subject::from_static("INBOX.67")),
                headers: Some({
                    let mut headers = HeaderMap::new();
                    headers.insert("Header", "Y");
                    headers
                }),
                payload: Bytes::from_static(b"Hello World"),
            }
        );

        let mut buf = BytesMut::from(&b"SUB hello.world 42\r\n"[..]);
        let msg = codec.decode(&mut buf).unwrap().unwrap();
        assert_eq!(
            msg,
            ClientOp::Subscribe {
                subject: Subject::from_static("hello.world"),
                sid: 42,
                queue_group: None,
            }
        );
        let mut buf = BytesMut::from(&b"SUB hello.world greeter 42\r\n"[..]);
        let msg = codec.decode(&mut buf).unwrap().unwrap();
        assert_eq!(
            msg,
            ClientOp::Subscribe {
                subject: Subject::from_static("hello.world"),
                sid: 42,
                queue_group: Some("greeter".to_string()),
            }
        );
    }
}
