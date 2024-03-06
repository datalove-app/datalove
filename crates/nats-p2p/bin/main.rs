use nats_p2p::{Config, Server};

use clap::Parser;
use rpassword::prompt_password;
use std::io;
use tracing::Level;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

struct Args {}

#[tokio::main]
async fn main() -> io::Result<()> {
    tracing_subscriber::fmt()
        .event_format(tracing_subscriber::fmt::format().with_target(false))
        .with_env_filter(EnvFilter::from_default_env())
        // .with_thread_ids(true)
        // .with_max_level(Level::TRACE)
        // .with_file(true)
        // .with_line_number(true)
        .init();

    let config = Config::default()
        .with_password_prompt(|| Ok(prompt_password("Enter the password for the SSH key:")?));
    let server = Server::new(config).await?;
    server.run().await?;

    Ok(())
}
