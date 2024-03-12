use crate::{
    cluster::ClusterInfo,
    core::{Protocol, Relay, ServerInfo, SessionManager},
    iroh::start_iroh,
    Config, Error,
};
use futures::{Future, TryStreamExt};
use iroh_net::{key::PublicKey, NodeAddr, NodeId};
use std::{
    net::SocketAddr,
    pin::Pin,
    sync::{atomic::AtomicU64, Arc},
    task::{Context, Poll},
    time::Duration,
};
use tokio::{net::TcpListener, task::JoinHandle};
use tokio_stream::wrappers::TcpListenerStream;

#[derive(Debug, Clone)]
pub struct ServerData {
    pk: PublicKey,
    config: Config,
    iroh: NodeAddr,
    // nat: jetstream::PeerInfo,
    server: ServerInfo,
    cluster: ClusterInfo,
}

impl ServerData {
    fn new(pk: PublicKey, config: Config) -> Self {
        let iroh = NodeAddr::from_parts(pk, None, vec![config.cluster_addr()]);

        let server = ServerInfo {
            server_name: config.name.clone(),
            host: config.host.to_string(),
            port: config.port,
            max_payload: config.max_payload as usize,
            server_id: iroh.node_id.to_string().to_uppercase(),

            connect_urls: vec![],
            client_id: 0,
            client_ip: "".to_string(),

            proto: Protocol::Dynamic as i8,
            version: "".to_string(),
            go: "".to_string(),
            nonce: "".to_string(),

            auth_required: false,
            tls_required: false,
            headers: true,
            lame_duck_mode: false,
        };

        let cluster = ClusterInfo {
            name: Some(config.cluster.name.clone()),
            leader: None,
            replicas: vec![],
        };
        Self {
            pk,
            config,
            iroh,
            server,
            cluster,
        }
    }

    fn with_host_addr(mut self, addr: SocketAddr) -> Self {
        self.config.host = addr.ip();
        self.config.port = addr.port();
        self.server.connect_urls = vec![format!("nats://{}", self.config.listen_addr())];
        // self.iroh.endpoints = vec![addr];
        self
    }

    fn client_url(&self) -> String {
        self.server.connect_urls[0].clone()
    }
    fn server_id(&self) -> String {
        self.iroh.node_id.to_string().to_uppercase()
    }

    fn log_start_message(&self) {
        let listen_addr = self.config.listen_addr();
        tracing::info!("Starting nats-p2p-server");
        tracing::info!("  Version:  0.0.1");
        tracing::info!("  Git:      [not set]");
        tracing::info!("  Name:     {}", self.config.name);
        tracing::info!("  ID:       {}", self.server_id());
        tracing::info!("Listening for client connections on {listen_addr}");
        tracing::info!("Server is ready");
    }
}

#[derive(Debug)]
pub struct Server {
    data: Arc<ServerData>,
    // iroh: IrohNode,
    listener_handle: Option<JoinHandle<()>>,
}

impl Server {
    pub async fn run() -> Result<Self, Error> {
        let config = Config::default();
        Self::run_config(config).await
    }

    pub async fn run_config(mut config: Config) -> Result<Self, Error> {
        // read ssh key
        let sk = config.read_ssh_key().await?;

        // start iroh
        // let (_, node_info) = start_iroh(&config, sk).await?;

        let listener = TcpListener::bind(config.listen_addr()).await?;
        let data =
            Arc::new(ServerData::new(sk.public(), config).with_host_addr(listener.local_addr()?));
        let relay = Relay::with_prefix(data.server_id()[0..6].to_string());

        Ok(Self {
            data: data.clone(),
            listener_handle: Some(tokio::spawn(async move {
                let data = data.clone();
                data.log_start_message();

                let _ = SessionManager::run(
                    relay,
                    data.server.clone(),
                    TcpListenerStream::new(listener),
                )
                .await;
            })),
        })
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        if let Some(handle) = self.listener_handle.take() {
            tracing::warn!("server dropped, shutting down");
            handle.abort();
        }
    }
}

impl Future for Server {
    type Output = Result<(), Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(handle) = self.listener_handle.as_mut() {
            match Pin::new(handle).poll(cx) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(Ok(())) => Poll::Ready(Ok(())),
                Poll::Ready(Err(e)) => Poll::Ready(Err(Error::server("listener error: {}", e))),
            }
        } else {
            Poll::Ready(Ok(()))
        }
    }
}

#[doc(hidden)]
pub mod server {
    use super::*;

    impl Server {
        pub fn client_url(&self) -> String {
            self.data.client_url()
        }
        pub fn client_port(&self) -> u16 {
            self.data.config.port
        }
    }

    pub fn run_basic_server() -> Server {
        run_server_with_port("", Some("0"))
    }

    pub fn run_server_with_port(_cfg: &str, port: Option<&str>) -> Server {
        let config = Config {
            name: "nats-p2p-test".to_string(),
            port: port.and_then(|p| p.parse().ok()).unwrap_or(0),
            ..Default::default()
        };
        futures::executor::block_on(Server::run_config(config))
            .expect("should not fail to start server")
    }
}
