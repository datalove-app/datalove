use crate::{
    cluster::ClusterInfo,
    core::{Protocol, Relay, ServerInfo, Session, SessionArgs},
    iroh::start_iroh,
    Config, Error,
};
use iroh_net::NodeAddr;
use ractor::Actor;
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::net::{TcpListener, TcpStream};

#[derive(Debug)]
pub struct Server {
    config: Config,
    // iroh: IrohNode,
    info: Info,
}

#[derive(Debug)]
pub struct Info {
    iroh: NodeAddr,
    // nat: jetstream::PeerInfo,
    cluster: ClusterInfo,
}

impl Server {
    pub fn client_url(&self) -> String {
        format!("nats://{}", self.config.listen_addr())
    }

    pub async fn new(mut config: Config) -> Result<Self, Error> {
        // read ssh key
        let sk = config.read_ssh_key().await?;

        // start iroh
        // let (_, node_info) = start_iroh(&config, sk).await?;

        let info = Info {
            iroh: NodeAddr::from_parts(sk.public(), None, vec![config.cluster_addr()]),
            // nat: jetstream::PeerInfo {
            //     name: config.name.clone(),
            //     current: true,
            //     active: Duration::from_secs(0),
            //     offline: true,
            //     lag: None,
            // },
            cluster: ClusterInfo {
                name: Some(config.cluster.name.clone()),
                leader: None,
                // TODO: config.cluster.authorized_keys,
                replicas: vec![],
            },
        };

        Ok(Self { config, info })
    }

    // #[tracing::instrument(skip(self))]
    pub async fn run(self) -> Result<(), Error> {
        let listener = TcpListener::bind(self.config.listen_addr()).await?;
        self.log_start_message();

        let relay = Relay::default();
        let mut client_id = 0u64;
        loop {
            client_id += 1;
            let host_addr = listener.local_addr()?;
            let (io, peer_addr) = listener.accept().await?;

            let server_info = self.server_info(client_id, peer_addr, host_addr);

            let relay = relay.clone();
            tokio::spawn(async move {
                let (_client, handle) = Actor::spawn(
                    Some(Session::<TcpStream>::name(client_id)),
                    Session::new(),
                    SessionArgs {
                        io,
                        inbox_prefix: None,
                        server_info,
                        relay,
                    },
                )
                .await
                .map_err(|e| Error::server("Session spawn error", e))?;

                handle
                    .await
                    .map_err(|e| Error::server("Session actor error", e))?;

                Ok::<(), Error>(())
            });
        }
    }

    fn log_start_message(&self) {
        tracing::info!("Starting iroh-nats-server");
        tracing::info!("  Version:  0.0.1");
        tracing::info!("  Git:      [not set]");
        tracing::info!("  Name:     {}", self.config.name);
        tracing::info!("  ID:       {}", self.info.server_id());
        tracing::info!(
            "Listening for client connections on {}",
            self.config.listen_addr()
        );
        tracing::info!("Server is ready");
    }

    // fn cluster_info(&self) -> &core::ClusterInfo {
    //     self.info.cluster
    // }

    fn server_info(
        &self,
        client_id: u64,
        client_addr: SocketAddr,
        host_addr: SocketAddr,
    ) -> ServerInfo {
        ServerInfo {
            server_id: self.info.server_id(),
            server_name: self.config.name.clone(),
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
            connect_urls: vec![host_addr.to_string()],

            client_id,
            client_ip: client_addr.to_string(),
            lame_duck_mode: false,
        }
    }
}

impl Info {
    pub fn server_id(&self) -> String {
        self.iroh.node_id.to_string().to_uppercase()
    }
}

pub fn run_basic_server() -> Server {
    todo!()
}
