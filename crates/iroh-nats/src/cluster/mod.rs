//! Cluster protocol, as documented [here](https://docs.nats.io/reference/reference-protocols/nats-server-protocol) where instead of a TCP/TLS socket, we use an [`iroh::MagicSocket`] with built-in e2ee over QUIC.

pub use async_nats::jetstream::stream::ClusterInfo;

use crate::{Client, Error, Message, Subject};
use futures::{
    future::Ready,
    task::{Context, Poll},
};
use std::{collections::BTreeMap, sync::Arc};

#[derive(Debug)]
pub struct Api {}

impl tower::Service<Message> for Api {
    type Response = Message;
    type Error = Error;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        unimplemented!()
    }

    fn call(&mut self, _req: Message) -> Self::Future {
        unimplemented!()
    }
}
