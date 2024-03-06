//! Core NATS wire codec, types and services.

mod codec;
mod pubsub;
mod session;

pub(crate) use pubsub::{Relay, SubscriberId};

pub use async_nats::{ConnectInfo, Protocol, ServerInfo, Subject};
pub use codec::Codec;
pub use session::{Session, SessionArgs};

// #[derive(Debug)]
// pub struct Api {
//     subs: BTreeMap<Subject, Arc<Client>>,
// }

// impl tower::Service<CoreOp> for Api {
//     type Response = Message;
//     type Error = Error;
//     type Future = Ready<Result<Self::Response, Self::Error>>;

//     fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         Poll::Ready(Ok(()))
//     }

//     fn call(&mut self, msg: CoreOp) -> Self::Future {
//         // match msg {
//         //     core::CoreMessage::Ping => {
//         //         res_sink.send(core::CoreMessage::Pong).await?;
//         //     }
//         //     core::CoreMessage::Pong => {
//         //         res_sink.send(core::CoreMessage::Ping).await?;
//         //     }
//         //     _ => {}
//         // }
//         unimplemented!()
//     }
// }

struct Connection {}
