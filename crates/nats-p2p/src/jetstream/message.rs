use crate::{
    core::{ClientOp, CoreMessage, ServerOp},
};
use bytes::Bytes;


pub const INFO: Bytes = Bytes::from_static(b"$JS.API.INFO");

// streams
pub const STREAM_LIST: Bytes = Bytes::from_static(b"$JS.API.STREAM.LIST");
pub const STREAM_NAMES: Bytes = Bytes::from_static(b"$JS.API.STREAM.NAMES");
pub const STREAM_CREATE: Bytes = Bytes::from_static(b"$JS.API.STREAM.CREATE.");
pub const STREAM_UPDATE: Bytes = Bytes::from_static(b"$JS.API.STREAM.UPDATE.");
pub const STREAM_INFO: Bytes = Bytes::from_static(b"$JS.API.STREAM.INFO.");
pub const STREAM_DELETE: Bytes = Bytes::from_static(b"$JS.API.STREAM.DELETE.");
pub const STREAM_PURGE: Bytes = Bytes::from_static(b"$JS.API.STREAM.PURGE.");
pub const STREAM_MSG_CREATE: Bytes = Bytes::from_static(b"$JS.API.STREAM.MSG.GET.");
pub const STREAM_MSG_UPDATE: Bytes = Bytes::from_static(b"$JS.API.STREAM.MSG.DELETE.");
pub const STREAM_SNAPSHOT: Bytes = Bytes::from_static(b"$JS.API.STREAM.SNAPSHOT.");
pub const STREAM_RESTORE: Bytes = Bytes::from_static(b"$JS.API.STREAM.RESTORE.");

// consumers
pub const CONSUMER_CREATE: Bytes = Bytes::from_static(b"$JS.API.CONSUMER.CREATE.<stream>");
pub const CONSUMER_DURABLE_CREATE: Bytes =
    Bytes::from_static(b"$JS.API.CONSUMER.DURABLE.CREATE.<stream>.<consumer>");
pub const CONSUMER_DELETE: Bytes =
    Bytes::from_static(b"$JS.API.CONSUMER.DELETE.<stream>.<consumer>");
pub const CONSUMER_INFO: Bytes = Bytes::from_static(b"$JS.API.CONSUMER.INFO.<stream>.<consumer>");
pub const CONSUMER_LIST: Bytes = Bytes::from_static(b"$JS.API.CONSUMER.LIST.<stream>");
pub const CONSUMER_MSG: Bytes =
    Bytes::from_static(b"$JS.API.CONSUMER.MSG.NEXT.<stream>.<consumer>");
pub const CONSUMER_NAMES: Bytes = Bytes::from_static(b"$JS.API.CONSUMER.NAMES.<stream>");

impl CoreMessage {
    pub fn is_jetstream(&self) -> bool {
        match self {
            CoreMessage::Inbound(op, _) => op.is_jetstream(),
            CoreMessage::Outbound(op) => op.is_jetstream(),
            _ => false,
        }
    }
}

impl ClientOp {
    pub fn is_jetstream(&self) -> bool {
        false
    }
}

impl ServerOp {
    pub fn is_jetstream(&self) -> bool {
        false
    }
}
