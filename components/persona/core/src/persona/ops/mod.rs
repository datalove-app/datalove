mod ops;

use super::State;
use crate::{device::DeviceLogProof, util, Device, Error, Weight};
use borsh::{BorshDeserialize, BorshSerialize};
use cid::Cid;
use ed25519_dalek::{Signature, Signer};
use risc0_zkvm::Receipt;

/// A single signer (device) involved in an [`Operation`].
pub type OperationSigner = (Device, DeviceLogProof);

/// The state of the signers involved in a [`SignedOperation`]
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct OperationSigners(Vec<OperationSigner>);

///
#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct SignedOperation {
    operation: ops::Operation,
    #[borsh(
        deserialize_with = "util::ed25519::deserialize_signatures",
        serialize_with = "util::ed25519::serialize_signatures"
    )]
    signatures: Vec<Signature>,
}

impl SignedOperation {
    pub fn genesis<Si: Signer<Signature>>(metadata: Cid) -> Result<Self, Error> {
        // let op = GenesisOperation {
        //     high_threshold,
        //     low_threshold,
        //     peer,
        //     metadata,
        // };

        todo!()
    }
}
