use datalove_app::App;

#[tokio::main]
async fn main() {
    #[cfg(target_family = "unix")]
    blitz::launch(App).await;
}
