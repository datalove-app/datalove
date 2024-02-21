use crate::{core::Api as CoreApi, jetstream::Api as JetstreamApi};
use tokio::net::TcpStream;

#[derive(Debug)]
pub struct Client {
    tcp: TcpStream,
}

impl Client {
    pub fn new(tcp: TcpStream) -> Self {
        Self { tcp }
    }
}

#[derive(Debug)]
pub struct Api {
    core: CoreApi,
    jetstream: JetstreamApi,
}
