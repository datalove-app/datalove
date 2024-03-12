use crate::{
    cluster::ClusterInfo,
    core::{Protocol, ServerInfo},
    Error,
};
use core::fmt;
use dirs::home_dir;
use iroh_net::{
    // config::Endpoint,
    key::{PublicKey, SecretKey},
};
use serde::{Deserialize, Serialize};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
};

///
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    /// // #[arg(short, long, default_value = "Config::DEFAULT_SERVER_NAME")]
    #[serde(default = "Config::default_server_name")]
    pub name: String,

    /// // #[arg(short = "a", long = "addr", default_value = "Config::DEFAULT_LISTEN_ADDR")]
    #[serde(default = "Config::default_host")]
    pub host: IpAddr,

    /// // #[arg(short, long, default_value = "Config::DEFAULT_LISTEN_PORT")]
    #[serde(default = "Config::default_port")]
    pub port: u16,

    #[serde(default = "Config::default_max_payload")]
    pub max_payload: usize,

    ///
    #[serde(default)]
    pub cluster: ClusterConfig,

    ///
    #[serde(default)]
    pub jetstream: Option<JetStreamConfig>,
}

impl Config {
    const DEFAULT_SERVER_NAME: &'static str = "nats-p2p";
    const DEFAULT_LISTEN_ADDR: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);
    const DEFAULT_LISTEN_PORT: u16 = 4222;
    const DEFAULT_MAX_PAYLOAD: usize = 65535;

    pub fn default_server_name() -> String {
        Self::DEFAULT_SERVER_NAME.to_string()
    }
    pub const fn default_host() -> IpAddr {
        Self::DEFAULT_LISTEN_ADDR
    }
    pub const fn default_port() -> u16 {
        Self::DEFAULT_LISTEN_PORT
    }
    pub const fn default_max_payload() -> usize {
        Self::DEFAULT_MAX_PAYLOAD
    }

    pub fn listen_addr(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.port)
    }
    pub fn cluster_addr(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.cluster.port)
    }

    /// Generates the node's Ed25519 key, or reads it from the filesystem, decrypting it with a callback-provided password if necessary.
    pub async fn read_ssh_key(&mut self) -> Result<SecretKey, Error> {
        use ssh_key::private;
        use tokio::fs;
        use zeroize::Zeroizing;

        if self.cluster.ssh_key_path.is_none() {
            return Ok(SecretKey::generate());
        }

        let path = self.cluster.ssh_key_path.as_ref().unwrap();
        let secret_key_file = Zeroizing::new(fs::read_to_string(path).await?);

        let mut secret_key = private::PrivateKey::from_openssh(&*secret_key_file)?;
        if secret_key.is_encrypted() {
            let password = Zeroizing::new(rpassword::prompt_password(
                "Enter the password for the SSH key:",
            )?);
            secret_key = secret_key.decrypt(password.as_bytes())?;
        }

        match secret_key.key_data() {
            private::KeypairData::Ed25519(key) => Ok(SecretKey::from(key.private.to_bytes())),
            _ => Err(ssh_key::Error::FormatEncoding.into()),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            name: Self::default_server_name(),
            host: Self::default_host(),
            port: Self::default_port(),
            max_payload: Self::default_max_payload(),
            cluster: Default::default(),
            jetstream: Default::default(),
        }
    }
}

///
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClusterConfig {
    /// Name of the cluster to create/join.
    #[serde(default = "ClusterConfig::default_name")]
    pub name: String,

    // /// Ed25519 secret key to use as the node's network address.
    // #[serde(default)]
    // pub ssh_key: Option<String>,
    /// Path to the OpenSSH-formatted Ed25519 private key. See [`ClusterConfig::ssh_key`].
    #[serde(default = "ClusterConfig::default_client_ssh_key")]
    pub ssh_key_path: Option<PathBuf>,

    /// Path to the OpenSSH-formatted `authorized_keys` file to use as the cluster's bootstrap peers.
    #[serde(default = "ClusterConfig::default_authorized_keys")]
    pub authorized_keys_path: Option<PathBuf>, // path

    /// Listening port for cluster messages (currently, over [`iroh`] QUIC).
    #[serde(default = "ClusterConfig::default_port")]
    pub port: u16,
    // routes, advertise, listen?
}

impl ClusterConfig {
    pub fn default_name() -> String {
        "default".to_string()
    }
    pub fn default_client_ssh_key() -> Option<PathBuf> {
        Some(home_dir().unwrap().join(".ssh/id_ed25519"))
    }
    pub fn default_authorized_keys() -> Option<PathBuf> {
        Some(home_dir().unwrap().join(".ssh/authorized_keys"))
    }
    pub fn default_port() -> u16 {
        6222
    }
}

impl Default for ClusterConfig {
    fn default() -> Self {
        Self {
            name: Self::default_name(),
            ssh_key_path: None,
            authorized_keys_path: None,
            port: Self::default_port(),
        }
    }
}

// pub struct AuthorizedKey {
//     pub key: String,
//     pub remote_addr: Option<(IpAddr, u16)>,
// }

///
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JetStreamConfig {
    #[serde(default = "JetStreamConfig::default_store_dir")]
    pub store_dir: PathBuf,

    #[serde(default = "JetStreamConfig::default_max_mem")]
    pub max_mem: usize,

    #[serde(default = "JetStreamConfig::default_max_file")]
    pub max_file: usize,
    // pub max_streams: usize,
    // pub max_consumers: usize,
}

impl JetStreamConfig {
    const DEFAULT_MAX_MEMORY: usize = 1024 * 1024 * 1024;
    const DEFAULT_MAX_FILE: usize = 1024 * 1024 * 1024;

    fn default_store_dir() -> PathBuf {
        home_dir().unwrap().join(".nats-p2p/jetstream")
    }
    fn default_max_mem() -> usize {
        Self::DEFAULT_MAX_MEMORY
    }
    fn default_max_file() -> usize {
        Self::DEFAULT_MAX_FILE
    }
}

impl Default for JetStreamConfig {
    fn default() -> Self {
        Self {
            store_dir: Self::default_store_dir(),
            max_mem: Self::default_max_mem(),
            max_file: Self::default_max_file(),
        }
    }
}
