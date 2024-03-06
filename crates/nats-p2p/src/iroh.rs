use crate::{config::Config, Error};
use iroh::{
    bytes::store::{flat, mem},
    net::key::SecretKey,
    node::Node,
    sync::store::{fs, memory},
    util::path::IrohPaths,
};
use iroh_net::NodeAddr;
use std::{net::SocketAddr, path::Path};

// pub type IrohNode = Node<BlobStore>;

pub async fn start_iroh(config: &Config, secret_key: SecretKey) -> Result<((), NodeAddr), Error> {
    let node_info = if let Some(js) = &config.jetstream {
        let blob_store = flat::Store::load(js.store_dir.join(&IrohPaths::BaoFlatStoreDir))
            .await
            .map_err(Error::Iroh)?;
        let doc_store =
            fs::Store::new(js.store_dir.join(&IrohPaths::DocsDatabase)).map_err(Error::Iroh)?;
        let node = Node::builder(blob_store, doc_store)
            .secret_key(secret_key)
            .bind_port(config.cluster_addr().port())
            .spawn()
            .await
            .map_err(Error::Iroh)?;
        node.my_addr().await.map_err(Error::Iroh)?
    } else {
        let blob_store = mem::Store::default();
        let doc_store = memory::Store::default();
        let node = Node::builder(blob_store, doc_store)
            .secret_key(secret_key)
            .bind_port(config.cluster_addr().port())
            .spawn()
            .await
            .map_err(Error::Iroh)?;
        node.my_addr().await.map_err(Error::Iroh)?
    };

    Ok(((), node_info))
}
