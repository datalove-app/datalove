pub mod consumer;

pub use async_nats::jetstream::stream::{
    Config as StreamConfig, ConsumerLimits, DiscardPolicy, PeerInfo, Republish, RetentionPolicy,
    SubjectTransform,
};

use crate::{Error, Message};
use futures::{
    future::Ready,
    task::{Context, Poll},
};

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
