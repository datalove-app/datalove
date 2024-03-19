use nats_p2p::*;


use tokio::io::{self};

use tracing_subscriber::{prelude::*, EnvFilter};

struct Args {}

#[hydroflow::main]
async fn main() -> io::Result<()> {
    let fmt = tracing_subscriber::fmt::layer()
        // .event_format(tracing_subscriber::fmt::format())
        .with_target(false)
        .with_ansi(true)
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

    // // TODO: load from file or args to know which SSH key to load
    // let config = Config::default();
    // let server = Server::run_config(config).await?;
    // server.await?;

    let config = Config::default();
    let _ = flow::server(config)
        .await?
        .run_async()
        .await
        .ok_or_else(|| io::Error::other("server error"))?;

    Ok(())
}
