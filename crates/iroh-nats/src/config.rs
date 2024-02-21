use crate::Error;
use dirs::home_dir;
use iroh_net::key::SecretKey;
use serde::{Deserialize, Serialize};
use std::{net::IpAddr, path::PathBuf, str::FromStr};

///
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(default = "Config::default_server_name")]
    pub name: String,
    #[serde(default = "Config::default_host")]
    pub host: IpAddr,
    #[serde(default = "Config::default_port")]
    pub port: u16,
    #[serde(default)]
    pub cluster: ClusterConfig,
    // #[serde(default)]
    pub jetstream: JetStreamConfig,
}

impl Config {
    pub fn default_server_name() -> String {
        "iroh-nats".to_string()
    }
    pub fn default_host() -> IpAddr {
        IpAddr::from([127, 0, 0, 1])
    }
    pub fn default_port() -> u16 {
        4222
    }

    pub fn listen_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub async fn read_ssh_key<F>(&self, prompt: F) -> Result<SecretKey, Error>
    where
        F: FnOnce(&str) -> Result<String, Error>,
    {
        use ssh_key::private;
        use tokio::fs;
        use zeroize::Zeroizing;

        if let Some(ref key) = self.cluster.ssh_key {
            return SecretKey::from_str(key).map_err(|_| ssh_key::Error::FormatEncoding.into());
        }

        let secret_key_file = Zeroizing::new(fs::read_to_string(&self.cluster.ssh_key_path).await?);

        let mut secret_key = private::PrivateKey::from_openssh(&*secret_key_file)?;
        if secret_key.is_encrypted() {
            let password = Zeroizing::new(prompt("Enter your SSH key password: ")?);
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
            cluster: Default::default(),
            jetstream: Default::default(),
        }
    }
}

///
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClusterConfig {
    #[serde(default = "ClusterConfig::default_name")]
    pub name: String,
    #[serde(default)]
    pub ssh_key: Option<String>,
    #[serde(default = "ClusterConfig::default_client_ssh_key")]
    pub ssh_key_path: PathBuf, // path
    #[serde(default = "ClusterConfig::default_authorized_keys")]
    pub authorized_keys_path: PathBuf, // path
}

impl ClusterConfig {
    pub fn default_name() -> String {
        "default".to_string()
    }
    pub fn default_client_ssh_key() -> PathBuf {
        home_dir().unwrap().join(".ssh/id_ed25519")
    }
    pub fn default_authorized_keys() -> PathBuf {
        home_dir().unwrap().join(".ssh/authorized_keys")
    }
}

impl Default for ClusterConfig {
    fn default() -> Self {
        Self {
            name: Self::default_name(),
            ssh_key: None,
            ssh_key_path: Self::default_client_ssh_key(),
            authorized_keys_path: Self::default_authorized_keys(),
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
    pub store_dir: PathBuf, // path
}

impl JetStreamConfig {
    fn default_store_dir() -> PathBuf {
        home_dir().unwrap().join(".iroh-nats/jetstream")
    }
}

impl Default for JetStreamConfig {
    fn default() -> Self {
        Self {
            store_dir: Self::default_store_dir(),
        }
    }
}
