use tokio::{io, process::Command};

fn setup_command(client_url: String) -> Command {
    let mut cmd = Command::new("nats");
    cmd.arg("--trace")
        .arg("server")
        .arg("check")
        .arg("-s")
        .arg(client_url);
    cmd
}

async fn run(args: &[&str]) -> io::Result<()> {
    let server = nats_p2p::run_basic_server();

    let mut cmd = setup_command(server.client_url());
    for arg in args {
        cmd.arg(arg);
    }

    let exit = cmd.spawn()?.wait().await?;
    assert!(exit.success());
    Ok(())
}

#[tokio::test]
async fn connection() -> io::Result<()> {
    run(&["connection"]).await
}

#[tokio::test]
#[ignore]
async fn stream() -> io::Result<()> {
    run(&["stream", "--name", "nats-p2p-test"]).await
}

#[tokio::test]
#[ignore]
async fn message() -> io::Result<()> {
    run(&["message", "--name", "nats-p2p-test"]).await
}

#[tokio::test]
#[ignore]
async fn meta() -> io::Result<()> {
    run(&["meta", "--name", "nats-p2p-test"]).await
}

#[tokio::test]
#[ignore]
async fn jetstream() -> io::Result<()> {
    run(&["jetstream", "--name", "nats-p2p-test"]).await
}

#[tokio::test]
#[ignore]
async fn server() -> io::Result<()> {
    run(&["server", "--name", "nats-p2p-test"]).await
}

#[tokio::test]
#[ignore]
async fn kv() -> io::Result<()> {
    run(&["kv", "--name", "nats-p2p-test"]).await
}

#[tokio::test]
#[ignore]
async fn credential() -> io::Result<()> {
    run(&["credential", "--name", "nats-p2p-test"]).await
}
