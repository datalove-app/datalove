use crate::{
    cluster::ClusterInfo,
    core::{Protocol, Relay, ServerInfo, Session, SessionArgs},
    iroh::start_iroh,
    Config, Error,
};
use futures::TryStreamExt;
use iroh_net::NodeAddr;
use std::{
    net::SocketAddr,
    sync::{atomic::AtomicU64, Arc},
    time::Duration,
};
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;

#[derive(Debug, Clone)]
pub struct Server {
    config: Config,
    info: Info,
    relay: Relay,
    // iroh: IrohNode,
}

#[derive(Debug, Clone)]
pub struct Info {
    iroh: NodeAddr,
    // nat: jetstream::PeerInfo,
    cluster: ClusterInfo,
}

impl Server {
    pub fn server_id(&self) -> String {
        self.info.iroh.node_id.to_string().to_uppercase()
    }
    pub fn client_url(&self) -> String {
        format!("nats://{}", self.config.listen_addr())
    }
    pub fn client_port(&self) -> u16 {
        self.config.port
    }

    pub async fn new() -> Result<Self, Error> {
        let config = Config::default();
        Self::from_config(config).await
    }

    pub async fn from_config(mut config: Config) -> Result<Self, Error> {
        let relay = Relay::default();

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

        Ok(Self {
            config,
            info,
            relay,
        })
    }

    // #[tracing::instrument(skip(self))]
    pub async fn run(&self) -> Result<(), Error> {
        let listener = TcpListener::bind(self.config.listen_addr()).await?;

        self.log_start_message();

        let client_id = AtomicU64::new(0);
        TcpListenerStream::new(listener)
            .try_for_each_concurrent(None, |io| async {
                let server_info = self.server_info(
                    client_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
                    io.peer_addr()?,
                    io.local_addr()?,
                );

                Session::spawn(io, self.relay.clone(), server_info)?;
                Ok(())
            })
            .await?;

        Ok(())
    }

    fn log_start_message(&self) {
        tracing::info!("Starting iroh-nats-server");
        tracing::info!("  Version:  0.0.1");
        tracing::info!("  Git:      [not set]");
        tracing::info!("  Name:     {}", self.config.name);
        tracing::info!("  ID:       {}", self.server_id());
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
            server_id: self.server_id(),
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
            connect_urls: vec![self.client_url()],

            client_id,
            client_ip: client_addr.to_string(),
            lame_duck_mode: false,
        }
    }
}

// pub fn run_basic_server() -> Arc<Server> {
//     let config = Config::default();
//     let server = futures::executor::block_on(async move { Server::new(config).await.unwrap() });
//     let task = Arc::new(server);
//     tokio::task::spawn(async move {
//         // let server = server.clone();
//         task.clone().run().await
//     });
//     task
// }
