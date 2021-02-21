//!
//! to look at:
//! - shards and their replica sets

use crate::dev::*;

pub trait Dataset {
    // type Cursor: Cursor;
    // type Entry;
    // type Query: Query<Self>;

    // async fn insert(&mut self, entry: Self::Entry);

    // async fn query(&self, query: Self::Query) -> Self::Cursor;
}

pub trait Sharding: ReplicaSet {
    // type Entry: Ord;

    // fn shard(&self, idx: D::Entry) -> &[PeerId];
}

pub trait ReplicaSet: PeerGroup {}

// #[async_trait_ext::async_trait_ext]
// pub trait Cursor {
//     type Dataset: Dataset

//     async fn next(&mut self) -> D::Entry;
// }

// pub trait Query<D: Dataset> {}
