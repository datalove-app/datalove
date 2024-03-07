use nats_p2p::{Config, Server};


use std::io;
use tracing_subscriber::{prelude::*, EnvFilter};

struct Args {}

#[tokio::main]
async fn main() -> io::Result<()> {
    let fmt = tracing_subscriber::fmt::layer()
        .event_format(tracing_subscriber::fmt::format().with_target(false))
        // .with_env_filter(EnvFilter::from_default_env())
        // .with_thread_ids(true)
        // .with_max_level(Level::TRACE)
        // .with_file(true)
        // .with_line_number(true);
        // .finish();
        ;

    tracing_subscriber::registry()
        .with(fmt)
        .with(EnvFilter::from_default_env())
        .init();

    // TODO: load from file or args to know which SSH key to load
    let config = Config::default();
    Server::from_config(config).await?.run().await?;

    Ok(())
}
