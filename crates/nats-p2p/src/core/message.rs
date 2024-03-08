pub use async_nats::{ConnectInfo, HeaderMap, Message, Protocol, ServerInfo, StatusCode};

use super::{QueueGroup, Subject, SubscriberId, WeightedQueueGroup};
use bytes::Bytes;
use std::fmt;

/// `ClientOp` represents all actions of `Client`.
///
/// [Original documentation](https://docs.nats.io/reference/reference-protocols/nats-protocol)
#[derive(Clone, Debug, Eq, PartialEq)]
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
        reply_to: Option<Subject>,
        headers: Option<HeaderMap>,
        payload: Bytes,
    },

    /// `SUB <subject> [queue group] <sid>`
    Subscribe {
        sid: u64,
        subject: Subject,
        queue_group: QueueGroup,
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
}

impl fmt::Display for ClientOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ctrl = self.control();
        match self {
            ClientOp::Ping | ClientOp::Pong => f.write_str(ctrl),
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
                reply_to,
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
#[derive(Clone, Debug, Eq, PartialEq)]
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
    /// `HMSG <subject> <sid> [reply-to] <#header-bytes> <#total-bytes>\r\n<version line>\r\n[headers]\r\n\r\n[payload]`
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
}

impl fmt::Display for ServerOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ctrl = self.control();
        match self {
            ServerOp::Ok | ServerOp::Ping | ServerOp::Pong => f.write_str(ctrl),
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

impl From<(SubscriberId, Message)> for ServerOp {
    fn from(((_, sid), msg): (SubscriberId, Message)) -> Self {
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
