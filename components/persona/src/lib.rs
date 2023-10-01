//! datalove-persona
//! ================
//! defines key actors and their commitments within the network
//!
//! ## General Idea:
//!
//! Blockchains are slow and expensive (relative to coordination-free,
//! self-hosted systems) largely because of the overhead of ensuring wide-area
//! data availability in order to maintain consensus. However, they work well
//! for many low-trust use cases because in their role as a decentralized
//! timestamping service, they 1. totally order all state changes and 2. as the
//! chain lengthens, previous state changes become increasingly committed into
//! the agreed-upon state of the chain.
//!
//! Peer-to-peer systems ... TODO:
//!
//! ## Concepts:
//!
//! 1. [`Device`]s:
//!     - singular keypair
//!         - never exposed to the user by default (even thru seed phrases)
//!         - can be rotated
//!         - optionally unlocked/activated by password/pin, biometrics
//!     - maintains a commit log of signed messages
//!         - known messages: user ops
//!         - abstract messages: anything serializable + hashable
//!             - generally a crdt op/delta-state
//! 2. [`User`]s (aka "personas"):
//!     - defined by a state machine specifying a set of devices responsible for
//!       it
//!         - device-specific thresholds define update invariants
//!     - commit log of zk proofs
//!         - verifies each device log's extension
//!         - verifies user-set state changes
//!         - publishes link to abstract metadata
//!
//! ## Proof System:
//!
//! Each [`Device`] maintains an append-only commit log (maintained by a
//! [`MerkleLog`]) of messages it might desire to persist or replicate across
//! the network. Devices initialize a [`User`] by creating it's genesis state,
//! committing that state to its log, and verifying the commitment within the
//! [`risc0`] zkvm, resulting in a receipt whose digest becomes the user's
//! [`DID`]. Subsequent user operations (to add or remove devices, change their
//! thresholds, etc) are added to each participating device's logs, and are
//! again applied to the user's state and verified within the zkvm (along with
//! the user's previous [`risc0_zkvm::Receipt`]), thus creating the user's
//! commit log.
//!
