use iroh_nats::{Config, Server};

use rpassword::prompt_password;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let config = Config::default();
    let sk = config
        .read_ssh_key(|prompt| Ok(prompt_password(prompt)?))
        .await?;
    Server::start(config, sk).await?;

    Ok(())
}
