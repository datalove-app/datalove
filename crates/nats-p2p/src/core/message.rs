pub use async_nats::{ConnectInfo, HeaderMap, Message, Protocol, ServerInfo, StatusCode};
use hydroflow::DemuxEnum;

use super::{QueueGroup, Subject, SubscriberId, WeightedQueueGroup};
use crate::Error;
use bytes::Bytes;
use ractor::RpcReplyPort;
use std::{fmt, net::SocketAddr};
use tokio_util::codec::{Decoder, Encoder};

/// Core API Message.
#[derive(Debug)]
pub enum CoreMessage {
    Inbound(ClientOp, Option<RpcReplyPort<()>>),
    Outbound(ServerOp),
}

impl CoreMessage {
    pub(crate) fn unwrap_inbound(self) -> ClientOp {
        match self {
            CoreMessage::Inbound(op, _) => op,
            _ => panic!("Expected ClientOp, got ServerOp"),
        }
    }
    pub(crate) fn unwrap_server(self) -> ServerOp {
        match self {
            CoreMessage::Outbound(op) => op,
            _ => panic!("Expected ServerOp, got ClientOp"),
        }
    }

    pub(crate) fn trace(&self, client_ip: SocketAddr, cid: u64) {
        match self {
            CoreMessage::Inbound(op, _) => op.trace(client_ip, cid),
            CoreMessage::Outbound(op) => op.trace(client_ip, cid),
        }
    }
}

impl From<ClientOp> for CoreMessage {
    fn from(op: ClientOp) -> Self {
        CoreMessage::Inbound(op, None)
    }
}
impl From<ServerOp> for CoreMessage {
    fn from(op: ServerOp) -> Self {
        CoreMessage::Outbound(op)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Codec(super::codec::Codec<ClientOp>);

impl Decoder for Codec {
    type Item = CoreMessage;
    type Error = Error;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        self.0.decode(src).map(|opt| opt.map(|op| op.into()))
    }
}

impl Encoder<CoreMessage> for Codec {
    type Error = Error;

    fn encode(&mut self, item: CoreMessage, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        match item {
            CoreMessage::Inbound(op, _) => self.0.encode(op, dst),
            CoreMessage::Outbound(op) => self.0.encode(op, dst),
        }
    }
}

/// `ClientOp` represents all actions of `Client`.
///
/// [Original documentation](https://docs.nats.io/reference/reference-protocols/nats-protocol)
#[derive(Clone, Debug, DemuxEnum, Eq, PartialEq)]
pub enum ClientOp {
    /// `CONNECT {["option_name":option_value],...}`
    Connect(Option<ConnectInfo>),

    /// `INFO {["option_name":option_value],...}`
    Info(Option<ServerInfo>),

    /// `PING`
    Ping,

    /// `PONG`
    Pong,

    /// `PUB <subject> [reply-to] <#bytes>\r\n[payload]`
    /// `HPUB <subject> [reply-to] <#header bytes> <#total bytes>\r\n[headers]\r\n\r\n[payload]`
    Publish {
        subject: Subject,
        reply: Option<Subject>,
        headers: Option<HeaderMap>,
        payload: Bytes,
    },

    /// `SUB <subject> [queue group] <sid>`
    Subscribe {
        subject: Subject,
        queue_group: QueueGroup,
        sid: u64,
        // sid: Subject,
    },

    /// `UNSUB <sid> [max_msgs]`
    Unsubscribe { sid: u64, max_msgs: Option<u64> },
}

impl ClientOp {
    const fn control(&self) -> &'static str {
        match self {
            Self::Connect(_) => "CONNECT",
            Self::Info(_) => "INFO",
            Self::Ping => "PING",
            Self::Pong => "PONG",
            Self::Publish { headers: None, .. } => "PUB",
            Self::Publish { .. } => "HPUB",
            Self::Subscribe { .. } => "SUB",
            Self::Unsubscribe { .. } => "UNSUB",
        }
    }

    pub(crate) fn trace(&self, client_ip: SocketAddr, cid: u64) {
        tracing::trace!(
            "{prefix} - {arrow} [{self}]",
            prefix = debug::trace_prefix(client_ip, cid),
            arrow = debug::arrow("<<-"),
        );
    }

    pub(crate) fn unwrap_into_message(self) -> Message {
        match self {
            ClientOp::Publish {
                subject,
                reply,
                headers,
                payload,
            } => Message {
                subject,
                reply,
                headers,
                status: None,
                description: None,
                length: payload.len(),
                payload,
            },
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for ClientOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ctrl = debug::ctrl(self.control());
        match self {
            ClientOp::Ping | ClientOp::Pong => write!(f, "{ctrl}"),
            ClientOp::Connect(info) => write!(
                f,
                "{ctrl} {}",
                serde_json::to_string(info).map_err(|_| fmt::Error)?
            ),
            ClientOp::Info(info) => write!(
                f,
                "{ctrl} {}",
                serde_json::to_string(info).map_err(|_| fmt::Error)?
            ),
            ClientOp::Publish {
                subject,
                reply: reply_to,
                payload,
                ..
            } => {
                write!(
                    f,
                    "{ctrl} {subject}{} {}",
                    reply_to.as_ref().map_or("".into(), |r| format!(" {}", r)),
                    payload.len(),
                )
            }
            ClientOp::Subscribe {
                sid,
                subject,
                queue_group,
            } => {
                write!(
                    f,
                    "{ctrl} {subject}{} {}",
                    queue_group
                        .as_ref()
                        .map_or("".into(), |q| format!(" {}", q)),
                    sid
                )
            }
            ClientOp::Unsubscribe { sid, max_msgs } => {
                write!(
                    f,
                    "{ctrl} {sid}{}",
                    max_msgs.as_ref().map_or("".into(), |m| format!(" {}", m))
                )
            }
        }
    }
}

/// [Original core documentation](https://docs.nats.io/reference/reference-protocols/nats-protocol)
/// [Original cluster documentation](https://docs.nats.io/reference/reference-protocols/nats-server-protocol)
#[derive(Clone, Debug, DemuxEnum, Eq, PartialEq)]
pub enum ServerOp {
    /// `CONNECT {["option_name":option_value],...}`
    Connect(ConnectInfo),

    /// `INFO {["option_name":option_value],...}`
    Info(ServerInfo),

    /// `PING`
    Ping,

    /// `PONG`
    Pong,

    /// +OK
    Ok,

    /// `-ERR <error message>`
    Err(String),

    /// `RS+ <account> <subject> [queue-group] [weight]
    Subscribe {
        account: String,
        subject: Subject,
        queue_group: WeightedQueueGroup,
    },

    /// `RS- <account> <subject>`
    Unsubscribe { account: String, subject: Subject },

    /// `MSG <subject> <sid> [reply-to] <#bytes>\r\n[payload]`
    /// `HMSG <subject> <sid> [reply-to] <#header-bytes> <#total-bytes>\r\n<version line> [status] [description]\r\n[headers]\r\n\r\n[payload]`
    /// `RMSG <account> <subject> [reply-to] <#bytes>\r\n[payload]`
    Message {
        sid: u64,
        account: Option<String>,
        subject: Subject,
        reply_to: Option<Subject>,
        headers: Option<HeaderMap>,
        status: Option<StatusCode>,
        description: Option<String>,
        payload: Bytes,
        // length: usize,
    },
}

impl ServerOp {
    const fn control(&self) -> &'static str {
        match self {
            Self::Connect(_) => "CONNECT",
            Self::Info(_) => "INFO",
            Self::Ping => "PING",
            Self::Pong => "PONG",
            Self::Ok => "+OK",
            Self::Err(_) => "-ERR",
            Self::Subscribe { .. } => "RS+",
            Self::Unsubscribe { .. } => "RS-",
            Self::Message { account, .. } if account.is_some() => "RMSG",
            Self::Message { headers: None, .. } => "MSG",
            Self::Message { .. } => "HMSG",
        }
    }

    pub(crate) fn trace(&self, client_ip: SocketAddr, id: impl Into<u64>) {
        tracing::trace!(
            "{prefix} - {arrow} [{self}]",
            prefix = debug::trace_prefix(client_ip, id.into()),
            arrow = debug::arrow("->>"),
        );
    }
}

impl fmt::Display for ServerOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ctrl = debug::ctrl(self.control());
        match self {
            ServerOp::Ok | ServerOp::Ping | ServerOp::Pong => write!(f, "{ctrl}"),
            ServerOp::Err(err) => write!(f, "{ctrl} {err}"),
            ServerOp::Connect(info) => write!(
                f,
                "{ctrl} {}",
                serde_json::to_string(info).map_err(|_| fmt::Error)?
            ),
            ServerOp::Info(info) => write!(
                f,
                "{ctrl} {}",
                serde_json::to_string(info).map_err(|_| fmt::Error)?
            ),
            ServerOp::Subscribe {
                account,
                subject,
                queue_group,
            } => {
                write!(
                    f,
                    "{ctrl} {account} {subject} {}",
                    queue_group
                        .as_ref()
                        .map_or("".into(), |(q, w)| format!(" {} {:?}", q, w))
                )
            }
            ServerOp::Unsubscribe { account, subject } => {
                write!(f, "{ctrl} {account} {subject}")
            }
            ServerOp::Message {
                sid,
                account,
                subject,
                reply_to,
                // headers,
                status,
                description,
                payload,
                ..
            } => {
                write!(
                    f,
                    "{ctrl} {subject}{}{}{}{} {}",
                    account
                        .as_ref()
                        .map_or(format!(" {sid}"), |a| format!(" {a}")),
                    reply_to.as_ref().map_or("".into(), |r| format!(" {r}")),
                    status.as_ref().map_or("".into(), |s| format!(" {s}")),
                    description.as_ref().map_or("".into(), |d| format!(" {d}")),
                    // headers,
                    payload.len(),
                )
            }
        }
    }
}

impl From<(StatusCode, Message)> for ServerOp {
    fn from((status, msg): (StatusCode, Message)) -> Self {
        Self::Message {
            sid: 0,
            account: None,
            subject: msg.subject,
            reply_to: msg.reply,
            headers: msg.headers,
            status: Some(status),
            description: msg.description,
            payload: msg.payload,
        }
    }
}

impl From<(SubscriberId, Message)> for ServerOp {
    fn from((sub_id, msg): (SubscriberId, Message)) -> Self {
        let (_, sid) = sub_id.to_parts();
        Self::from((sid, msg))
    }
}

impl From<(u64, Message)> for ServerOp {
    fn from((sid, msg): (u64, Message)) -> Self {
        Self::Message {
            sid,
            account: None,
            subject: msg.subject,
            reply_to: msg.reply,
            headers: msg.headers,
            status: msg.status,
            description: msg.description,
            payload: msg.payload,
        }
    }
}

impl From<(String, Message)> for ServerOp {
    fn from((account, msg): (String, Message)) -> Self {
        Self::Message {
            sid: 0,
            account: Some(account),
            subject: msg.subject,
            reply_to: msg.reply,
            headers: msg.headers,
            status: msg.status,
            description: msg.description,
            payload: msg.payload,
        }
    }
}

pub mod debug {
    use anstyle::{AnsiColor, Color, Style};
    use std::fmt;

    const CLIENT: Style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Cyan)));
    const CTRL: Style = Style::new()
        .fg_color(Some(Color::Ansi(AnsiColor::Green)))
        .bold();
    const ARROW: Style = Style::new()
        .fg_color(Some(Color::Ansi(AnsiColor::BrightBlue)))
        .bold();

    pub fn trace_prefix<T: fmt::Display>(ip: T, id: u64) -> String {
        format!("{CLIENT}{ip}{CLIENT:#} - {CLIENT}cid:{id}{CLIENT:#}")
    }

    pub fn ctrl(ctrl: &'static str) -> String {
        render(ctrl, CTRL)
    }

    pub fn arrow(arrow: &'static str) -> String {
        render(arrow, ARROW)
    }

    pub fn render<T: fmt::Display>(t: T, style: Style) -> String {
        format!("{style}{t}{style:#}")
    }
}
