pub use async_nats::{ConnectInfo, ServerInfo};

use crate::{Config, HeaderMap, StatusCode, Subject};
use bytes::Bytes;
use iroh_net::{NodeAddr, NodeId};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

/// A protocol operation sent by the client.
///
/// [Original documentation](https://docs.nats.io/reference/reference-protocols/nats-protocol)
#[derive(Clone, Debug)]
pub enum CoreMessage {
    /// `CONNECT {["option_name":option_value],...}`
    ///
    /// Sent by client.
    Connect(ConnectInfo),

    /// `INFO {["option_name":option_value],...}`
    ///
    /// Sent by server after initial connection, and periodically after that.
    Info(ServerInfo),

    /// `PUB <subject> [reply-to] <#bytes>\r\n[payload]`
    Pub {
        subject: Subject,
        reply_to: Option<Subject>,
        payload: Bytes,
    },

    /// `HPUB <subject> [reply-to] <#header bytes> <#total bytes>\r\n[headers]\r\n\r\n[payload]`
    Hpub {
        subject: Subject,
        reply_to: Option<Subject>,
        headers: HeaderMap,
        payload: Bytes,
    },

    /// `SUB <subject> [queue group] <sid>`
    Sub {
        subject: Subject,
        queue_group: Option<String>,
        sid: u64,
    },

    /// `UNSUB <sid> [max_msgs]`
    Unsub { sid: u64, max_msgs: Option<u64> },

    /// `MSG <subject> <sid> [reply-to] <#bytes>\r\n[payload]`
    Msg {
        subject: Subject,
        sid: u64,
        reply_to: Option<Subject>,
        payload: Bytes,
    },

    /// `HMSG <subject> <sid> [reply-to] <#header-bytes> <#total-bytes>\r\n<version line>\r\n[headers]\r\n\r\n[payload]`
    Hmsg {
        subject: Subject,
        headers: HeaderMap,
        sid: u64,
        reply_to: Option<Subject>,
        payload: Bytes,
    },

    /// `+OK`
    Ok,

    /// `-ERR <error message>`
    Err(String),

    /// `PING`
    Ping,

    /// `PONG`
    Pong,
}

impl CoreMessage {
    // pub(crate) fn serialize_server_info(
    //     info: &ServerInfo,
    //     buf: &mut BytesMut,
    // ) -> Result<(), Error> {
    //     serde_json::to_writer(buf.as_mut(), info).map_err(|e| Error::Io(e.into()))
    // }
}
