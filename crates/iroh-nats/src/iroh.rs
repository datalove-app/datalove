use crate::Error;
use iroh::{
    bytes::store::flat::Store as BlobStore, net::key::SecretKey, node::Node,
    sync::store::fs::Store as DocStore, util::path::IrohPaths,
};
use iroh_net::NodeAddr;
use std::path::Path;

pub type IrohNode = Node<BlobStore>;

pub async fn start_iroh(
    store_dir: &Path,
    secret_key: SecretKey,
) -> Result<(IrohNode, NodeAddr), Error> {
    let blob_store = BlobStore::load(store_dir.join(&IrohPaths::BaoFlatStoreDir))
        .await
        .map_err(Error::Iroh)?;

    let doc_store = DocStore::new(store_dir.join(&IrohPaths::DocsDatabase)).map_err(Error::Iroh)?;

    let node = Node::builder(blob_store, doc_store)
        .secret_key(secret_key)
        .spawn()
        .await
        .map_err(Error::Iroh)?;

    let node_info = node.my_addr().await.map_err(Error::Iroh)?;

    Ok((node, node_info))
}
