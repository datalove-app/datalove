//! Cluster protocol, as documented [here](https://docs.nats.io/reference/reference-protocols/nats-server-protocol) where instead of a TCP/TLS socket, we use an [`iroh::MagicSocket`] with built-in e2ee over QUIC.

pub use async_nats::jetstream::stream::ClusterInfo;
