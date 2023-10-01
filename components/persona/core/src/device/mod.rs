use crate::{util, Error};
use borsh::{BorshDeserialize, BorshSerialize};
use digest::Digest;
use ed25519_dalek::{
    ed25519::signature::KeypairRef, DigestSigner, DigestVerifier, Signature, VerifyingKey,
};
use merkle_log::{MerkleLog, Proof, Store};
use sha2::{Sha256, Sha512};

///
pub type DeviceKey = VerifyingKey;

///
pub type DeviceLogNode = [u8; 32];

///
pub type DeviceLog = MerkleLog<Sha256, DeviceLogNode>;

/// Combined proof data for verifying inclusion of both the previously-committed
/// and newly-committed heads in the device's log.
pub type DeviceLogProof = Proof<DeviceLogNode>;

///
#[derive(Copy, Clone, Debug, PartialEq, Eq, BorshDeserialize, BorshSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Device {
    #[borsh(
        deserialize_with = "util::ed25519::deserialize_device_key",
        serialize_with = "util::ed25519::serialize_device_key"
    )]
    pk: DeviceKey,
    log: DeviceLog,
}

impl Device {
    /// Domain separation seed.
    pub const SEED: &'static [u8] = b"datalove_persona::wallet";

    /// Initializes a new [`Device`] and it's [`MerkleLog`].
    pub fn init<Si>(pk: DeviceKey, signer: &Si) -> Result<Self, Error>
    where
        Si: DigestSigner<Sha512, Signature>,
    {
        let entry = {
            let entry = Self::entry_digest(pk.as_bytes());
            // assert that the signer is the owner of the public key being initialized
            let sig = signer.try_sign_digest(entry.clone())?;
            pk.verify_digest(entry, &sig)?;
            sig
        };

        Ok(Self {
            pk,
            log: MerkleLog::new(entry.to_bytes()),
        })
    }

    // /// Loads an initialized [`Device`] from its last known [`MerkleLog`], and the
    // /// [`Signature`] making up its current head entry.
    // pub fn load(pk: DeviceKey, log: DeviceLog, head: Signature) -> Result<Self, Error> {
    //     if pk.verify
    //     if log.head() != Sha256::leaf_digest(head.to_bytes()) {
    //         Err(Error::MerkleLogError(MerkleLogError::ProofError("invalid head")))
    //     }
    // }

    pub fn append<T, Si, St>(
        &mut self,
        entry: &T,
        signer: &Si,
        store: &mut St,
    ) -> Result<Signature, Error>
    where
        T: BorshSerialize,
        Si: DigestSigner<Sha512, Signature>,
        St: Store<DeviceLogNode>,
    {
        let entry_bytes = borsh::to_vec(entry)?;
        let entry = Self::entry_digest(&entry_bytes);
        let signature = signer.try_sign_digest(entry)?;
        self.log.append(signature.to_bytes(), store)?;
        Ok(signature)
    }

    pub fn public_key(&self) -> &DeviceKey {
        &self.pk
    }

    // /// Determines if this is a "null" peer, i.e. the default peer.
    // pub fn is_null_peer(&self) -> bool {
    //     self.pk.as_bytes() == &NULL_PEER_KEY
    // }

    pub(crate) fn entry_digest(entry: impl AsRef<[u8]>) -> Sha512 {
        Sha512::new_with_prefix(Self::SEED).chain_update(entry)
    }
}

impl AsRef<DeviceKey> for Device {
    fn as_ref(&self) -> &DeviceKey {
        self.public_key()
    }
}

impl AsRef<DeviceLog> for Device {
    fn as_ref(&self) -> &DeviceLog {
        &self.log
    }
}

impl KeypairRef for Device {
    type VerifyingKey = DeviceKey;
}

// impl Verifier<Signature> for Device {
//     fn verify(&self, msg: &[u8], signature: &Signature) -> Result<(), SignatureError> {
//         self.pk.verify(msg, signature)
//     }
// }

// pub(crate) const NULL_PEER_KEY: [u8; 32] = [
//     59, 106, 39, 188, 206, 182, 164, 45, 98, 163, 168, 208, 42, 111, 13, 115, 101, 50, 21, 119, 29,
//     226, 67, 166, 58, 192, 72, 161, 139, 89, 218, 41,
// ];

// /// A default [`Device`] with a secret key of all zeros.
// impl Default for Device {
//     fn default() -> Self {
//         let sk = ed25519_dalek::SigningKey::from_bytes(&[0u8; 32]);
//         let pk = sk.verifying_key();
//         Self::new(pk, &sk).expect("failed to create default peer")
//     }
// }

#[cfg(test)]
mod tests {
    // #[test]
    // fn default() {
    //     use super::*;
    //     let peer = Device::default();
    //     assert_eq!(peer.verifying_key().as_bytes(), &NULL_PEER_KEY);
    // }
}
