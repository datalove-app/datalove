use super::ClientId;
use crate::{
    core::{Message, Subject},
    Error,
};
use flume::{unbounded, Receiver, Sender};
use hydroflow::{hydroflow_syntax};
use std::num::NonZeroU16;
use tokio::task::spawn_local;

pub type Commander = Sender<(ClientId, Command)>;
pub type Responder = Receiver<(ClientId, Message)>;

#[derive(Debug, Clone)]
pub enum Command {
    Put(Message),
    Get(Subject),
    Del(Subject),
}

pub async fn run(num_threads: Option<NonZeroU16>) -> Result<(Commander, Responder), Error> {
    let (tx_ingress, rx_ingress) = unbounded();
    let (tx_egress, rx_egress) = unbounded();

    let mut hf = hydroflow_syntax! {
        /******************** command inbound ********************/
        commands = source_stream(rx_ingress.into_stream())
            -> demux(|v, var_args!(Put, Get, Del)| match v {
                (id, Command::Put(msg)) => Put.give((id, msg)),
                (id, Command::Get(sub)) => Get.give((id, sub)),
                (id, Command::Del(sub)) => Del.give((id, sub)),
            });

        // -- put
        commands[Put] -> null();

        // -- get
        commands[Get] -> null();

        // -- del
        commands[Del] -> null();

        /******************** response outbound ********************/
        responses = union()
            -> dest_sink(tx_egress.into_sink());
    };

    spawn_local(async move {
        hf.run_async().await;
    });

    Ok((tx_ingress, rx_egress))
}
