//! listen for client connections
//!     each has subs
//!     each sends commands
//! to iroh actor
//! route iroh doc/blob/msg events to clients

pub(crate) mod cluster;
pub(crate) mod codec;
pub(crate) mod core;
mod iroh;
pub(crate) mod jetstream;

pub mod api;
pub mod config;
pub mod error;

pub use crate::{api::Client, config::Config, error::Error, iroh::IrohNode};
pub use async_nats::{HeaderMap, Message, Protocol, StatusCode, Subject};

use dirs::home_dir;
use futures::{SinkExt, StreamExt, TryStreamExt};
use iroh::start_iroh;
use iroh_net::{key::SecretKey, NodeAddr};
use std::time::Duration;
use tokio::{io, net::TcpListener};
use tokio_util::codec::{Decoder, Encoder, Framed};

pub struct Server {
    config: Config,
    iroh: IrohNode,
    info: ServerInfo,
}

pub struct ServerInfo {
    iroh: NodeAddr,
    // nat: jetstream::PeerInfo,
    cluster: cluster::ClusterInfo,
}

impl Server {
    pub async fn start(config: Config, sk: SecretKey) -> Result<(), Error> {
        // start iroh
        let (iroh, node_info) = start_iroh(&config.jetstream.store_dir, sk).await?;

        let info = ServerInfo {
            iroh: node_info,
            // nat: jetstream::PeerInfo {
            //     name: config.name.clone(),
            //     current: true,
            //     active: Duration::from_secs(0),
            //     offline: true,
            //     lag: None,
            // },
            cluster: cluster::ClusterInfo {
                name: Some(config.cluster.name.clone()),
                leader: None,
                // replicas: config.cluster.authorized_keys,
                replicas: vec![],
            },
        };

        Self { config, iroh, info }.run().await
    }

    async fn run(self) -> Result<(), Error> {
        // start client service
        let listener = TcpListener::bind(self.config.listen_addr()).await?;

        let mut client_id = 0u64;
        loop {
            let (tcp_stream, _listen_addr) = listener.accept().await?;

            // TODO: create client service
            client_id += 1;
            let (mut res_sink, mut cmd_steam) =
                Framed::new(tcp_stream, codec::CoreCodec::default()).split();

            // send server info on connect
            res_sink
                .send(core::CoreMessage::Info(self.server_info(client_id)))
                .await?;

            while let Some(msg) = cmd_steam.try_next().await? {
                match msg {
                    core::CoreMessage::Ping => {
                        res_sink.send(core::CoreMessage::Pong).await?;
                    }
                    core::CoreMessage::Pong => {
                        res_sink.send(core::CoreMessage::Ping).await?;
                    }
                    _ => {}
                }
            }
        }
    }

    // fn cluster_info(&self) -> &core::ClusterInfo {
    //     self.info.cluster
    // }

    fn server_info(&self, client_id: u64) -> core::ServerInfo {
        core::ServerInfo {
            server_id: self.info.iroh.node_id.to_string(),
            server_name: Config::default_server_name(),
            host: self.config.host.to_string(),
            port: self.config.port,
            max_payload: 65535,
            auth_required: false,
            tls_required: false,
            headers: true,

            proto: Protocol::Dynamic as i8,
            version: "".to_string(),
            go: "".to_string(),
            nonce: "".to_string(),
            connect_urls: vec![self.config.listen_addr()],
            client_id,
            client_ip: "".to_string(),
            lame_duck_mode: false,
        }
    }
}
