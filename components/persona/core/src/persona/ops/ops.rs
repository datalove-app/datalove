use super::{OperationSigner, OperationSigners, State};
use crate::{device::DeviceLogProof, util, Device, Error, Weight};
use borsh::{BorshDeserialize, BorshSerialize};
use cid::Cid;
use ed25519_dalek::{Signature, Signer};
use risc0_zkvm::Receipt;

/// An operation to be applied to a persona's [`State`].
#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub enum Operation {
    Genesis(GenesisOperation),
    Bump(BumpOperation),
}

/// The genesis operation, that creates a new [`User`];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct GenesisOperation {
    initial_device: OperationSigner,
    #[borsh(
        deserialize_with = "util::cid::deserialize_cid",
        serialize_with = "util::cid::serialize_cid"
    )]
    metadata: Cid,
}

///
#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct ContinueOperation<T> {
    inner: T,
    signers: OperationSigners,
    #[borsh(
        deserialize_with = "util::cid::deserialize_cid",
        serialize_with = "util::cid::serialize_cid"
    )]
    metadata: Cid,
    #[borsh(
        deserialize_with = "util::risc0::deserialize_receipt",
        serialize_with = "util::risc0::serialize_receipt"
    )]
    prev: Receipt,
}

///
pub type BumpOperation = ContinueOperation<()>;
