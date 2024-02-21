mod info;

pub use info::{ConnectInfo, CoreMessage, ServerInfo};

use crate::{Client, Error, HeaderMap, Message, Subject};
use bytes::Bytes;
use futures::{
    future::Ready,
    task::{Context, Poll},
};
use iroh_net::NodeId;
use std::{collections::BTreeMap, sync::Arc};

#[derive(Debug)]
pub struct Api {
    subs: BTreeMap<Subject, Arc<Client>>,
}

impl tower::Service<CoreMessage> for Api {
    type Response = Message;
    type Error = Error;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        unimplemented!()
    }

    fn call(&mut self, _req: CoreMessage) -> Self::Future {
        unimplemented!()
    }
}
