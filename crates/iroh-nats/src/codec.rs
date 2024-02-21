// use std::io::prelude::*;
use crate::{
    core::{ConnectInfo, CoreMessage, ServerInfo},
    Error, HeaderMap, Subject,
};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use serde::{Deserialize, Serialize};
use serde_json::from_slice;
use std::{
    io::{self, BufRead, ErrorKind, Write},
    str::{self, FromStr},
};
use tokio_util::codec::{AnyDelimiterCodecError, Decoder, Encoder};

#[derive(Clone, Debug, Default)]
pub struct CoreCodec {
    inner: util::ChunkCodec,
}

impl Decoder for CoreCodec {
    type Item = CoreMessage;
    type Error = Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut chunk = match self.inner.decode(buf)? {
            None => return Ok(None),
            Some(chunk) => chunk,
        };

        if chunk.starts_with(b"+OK") {
            return Ok(Some(CoreMessage::Ok));
        }
        if chunk.starts_with(b"PING") {
            return Ok(Some(CoreMessage::Ping));
        }
        if chunk.starts_with(b"PONG") {
            return Ok(Some(CoreMessage::Pong));
        }

        if chunk.starts_with(b"INFO ") {
            chunk.advance(b"INFO ".len());
            let info =
                from_slice(chunk.as_ref()).map_err(|e| AnyDelimiterCodecError::Io(e.into()))?;

            return Ok(Some(CoreMessage::Info(info)));
        }

        Ok(None)
    }
}

impl Encoder<CoreMessage> for CoreCodec {
    type Error = Error;

    fn encode(&mut self, item: CoreMessage, buf: &mut BytesMut) -> Result<(), Self::Error> {
        match item {
            CoreMessage::Ok => self.inner.encode(&b"+OK"[..], buf)?,
            CoreMessage::Ping => self.inner.encode(&b"PING"[..], buf)?,
            CoreMessage::Pong => self.inner.encode(&b"PONG"[..], buf)?,
            CoreMessage::Info(ref info) => {
                buf.reserve(1024);
                buf.put_slice(b"INFO ");

                let mut ser = serde_json::Serializer::new(buf.as_mut());
                info.serialize(&mut ser)
                    .map_err(|e| AnyDelimiterCodecError::Io(e.into()))?;

                buf.put_slice(util::ChunkCodec::DELIMITER.as_bytes());
            }
            _ => unimplemented!(),
        };

        Ok(())
    }
}

pub enum ClusterOp {
    /// `INFO {["option_name":option_value],...}`
    Info(ServerInfo),

    /// `CONNECT {["option_name":option_value],...}`
    Connect(ConnectInfo),

    /// `RS+ <account> <subject> <queue> <weight>\r`
    RSub {
        account: u64,
        subject: Subject,
        queue: Option<Subject>,
        weight: Option<u32>,
    },

    /// `RS- <account> <subject>\r`
    RUnsub {
        account: u64,
        subject: Subject,
    },

    /// `RMSG <account> <subject> [reply-to] <# bytes>\r\n[payload]\r`
    RMsg {
        account: u64,
        subject: Subject,
        reply_to: Option<Subject>,
        payload: Bytes,
    },

    /// `PING\r`
    Ping,

    /// `PONG\r`
    Pong,

    Err(String),
}

mod util {
    use super::*;
    use core::{
        cmp,
        fmt::{self, Write},
    };
    use memchr::memmem;
    use tokio_util::codec::{AnyDelimiterCodecError, Decoder, Encoder};

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

        /// Are we currently discarding the remainder of a chunk which was over
        /// the length limit?
        is_discarding: bool,
    }

    impl ChunkCodec {
        pub const DELIMITER: &'static str = "\r\n";

        pub fn with_max_length(mut self, max_length: usize) -> Self {
            self.max_length = max_length;
            self
        }
    }

    impl Default for ChunkCodec {
        fn default() -> Self {
            Self {
                next_index: 0,
                max_length: usize::MAX,
                is_discarding: false,
            }
        }
    }

    impl Decoder for ChunkCodec {
        type Item = Bytes;
        type Error = Error;

        fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
            loop {
                // Determine how far into the buffer we'll search for a delimiter. If
                // there's no max_length set, we'll read to the end of the buffer.
                let read_to = cmp::min(self.max_length.saturating_add(1), buf.len());

                let new_chunk_offset =
                    memmem::find(&buf[self.next_index..read_to], Self::DELIMITER.as_bytes());

                match (self.is_discarding, new_chunk_offset) {
                    (false, Some(offset)) => {
                        // Found a chunk!
                        let new_chunk_index = offset + self.next_index;
                        self.next_index = 0;
                        let mut chunk = buf.split_to(new_chunk_index + Self::DELIMITER.len());
                        chunk.truncate(chunk.len() - Self::DELIMITER.len());
                        let chunk = chunk.freeze();
                        return Ok(Some(chunk));
                    }
                    (false, None) if buf.len() > self.max_length => {
                        // Reached the maximum length without finding a
                        // new chunk, return an error and start discarding on the
                        // next call.
                        self.is_discarding = true;
                        return Err(AnyDelimiterCodecError::MaxChunkLengthExceeded)?;
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
                        buf.advance(offset + self.next_index + Self::DELIMITER.len());
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
            todo!()
        }
    }

    impl Encoder<&[u8]> for ChunkCodec {
        type Error = io::Error;

        fn encode(&mut self, item: &[u8], buf: &mut BytesMut) -> Result<(), Self::Error> {
            buf.reserve(item.len() + Self::DELIMITER.len());
            buf.put(item);
            buf.put_slice(Self::DELIMITER.as_bytes());
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
            self.encode(item.as_ref(), buf)
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
            write!(buf, "{}{}", item, Self::DELIMITER)
                .map_err(|_| io::Error::other("fmt error"))?;
            Ok(())
        }
    }

    #[test]
    fn test_decode() -> io::Result<()> {
        let mut codec = ChunkCodec::default();
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"hello\r\nworld\r\n");

        let chunk = codec.decode(&mut buf)?.unwrap();
        assert_eq!(chunk.as_ref(), b"hello".as_ref());
        let chunk = codec.decode(&mut buf)?.unwrap();
        assert_eq!(chunk.as_ref(), b"world".as_ref());

        Ok(())
    }

    #[test]
    fn test_encode() -> io::Result<()> {
        let mut codec = ChunkCodec::default();
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"hello\r\nworld\r\n");

        codec.encode(b"hello".as_ref(), &mut buf)?;
        codec.encode(b"world".as_ref(), &mut buf)?;
        assert_eq!(buf.as_ref(), b"hello\r\nworld\r\n".as_ref());

        Ok(())
    }
}
