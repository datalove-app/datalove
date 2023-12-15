use crate::{util, Error};
use borsh::{BorshDeserialize, BorshSerialize};
use digest::{typenum::U64, Digest};
use ed25519_dalek::{
    Signature as Ed25519Signature, SigningKey as Ed25519SigningKey,
    VerifyingKey as Ed25519VerifyingKey,
};
use sha2::Sha512;
use signature::{DigestSigner, DigestVerifier, Error as SignatureError, Verifier};

///
#[derive(Copy, Clone, Debug, Eq, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Device {
    inner: DeviceInner,
    // log: MerkleLog<DeviceLogNode>,
}

impl Device {
    pub fn id(&self) -> [u8; 32] {
        match self.inner {
            DeviceInner::Ed25519(pk) => pk.to_bytes(),
        }
    }
}

///
#[derive(Copy, Clone, Debug, Eq, PartialEq, BorshDeserialize, BorshSerialize)]
#[borsh(use_discriminant = true)]
#[non_exhaustive]
enum DeviceInner {
    Ed25519(
        #[borsh(
            deserialize_with = "util::ed25519::deserialize_key",
            serialize_with = "util::ed25519::serialize_key"
        )]
        Ed25519VerifyingKey,
    ),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, BorshDeserialize, BorshSerialize)]
#[borsh(use_discriminant = true)]
#[non_exhaustive]
pub enum DeviceSignature {
    Ed25519(
        #[borsh(
            deserialize_with = "util::ed25519::deserialize_signature",
            serialize_with = "util::ed25519::serialize_signature"
        )]
        Ed25519Signature,
    ),
}

impl Device {
    /// Domain separation context.
    pub const SIGNING_CONTEXT: &[u8] = b"datalove::persona";

    // /// Initializes a new [`Device`] and it's [`MerkleLog`].
    // pub fn init<Si>(pk: Ed25519VerifyingKey, signer: &Si) -> Result<Self, Error>
    // where
    //     Si: DigestSigner<Sha512, DeviceSignature>,
    // {
    //     let entry = {
    //         let entry = Self::protocol_message_digest(pk.as_bytes());
    //         // assert that the signer is the owner of the public key being initialized
    //         let DeviceSignature::Ed25519(sig) = signer
    //             .try_sign_digest(entry.clone())
    //             .map_err(Error::SignatureError)?;
    //         pk.verify_digest(entry, &sig)
    //             .map_err(Error::SignatureError)?;
    //         sig
    //     };

    //     Ok(Self::Ed25519(pk))
    // }

    /*
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
        let entry = Self::protocol_message_digest(&entry_bytes);
        let signature = signer.try_sign_digest(entry)?;
        self.log.append(signature.to_bytes(), store)?;
        Ok(signature)
    }
     */

    // /// Determines if this is a "null" peer, i.e. the default peer.
    // pub fn is_null_peer(&self) -> bool {
    //     self.pk.as_bytes() == &NULL_PEER_KEY
    // }

    pub fn sign_message<D, Si>(&self, message: &[u8], signer: &Si) -> Result<DeviceSignature, Error>
    where
        D: Digest<OutputSize = U64> + Clone,
        Si: DigestSigner<D, DeviceSignature>,
    {
        let msg_digest = Self::protocol_message_digest::<D>(message);
        let sig = signer.try_sign_digest(msg_digest.clone())?;
        self.verify_digest::<D>(msg_digest, &sig)?;
        Ok(sig)
    }

    /// assumes we're verifying a protocol message digest
    fn verify_digest<D>(&self, msg_digest: D, signature: &DeviceSignature) -> Result<(), Error>
    where
        D: Digest<OutputSize = U64>,
    {
        match (self.inner, signature) {
            (DeviceInner::Ed25519(pk), DeviceSignature::Ed25519(sig)) => {
                Ok(pk.verify_digest(msg_digest, sig)?)
            }
        }
    }

    fn protocol_message_digest<D>(entry: impl AsRef<[u8]>) -> D
    where
        D: Digest<OutputSize = U64>,
    {
        D::new_with_prefix(Self::SIGNING_CONTEXT).chain_update(entry)
    }
}

// impl AsRef<Ed25519VerifyingKey> for Device {
//     fn as_ref(&self) -> &Ed25519VerifyingKey {
//         self.public_key()
//     }
// }

// impl KeypairRef for Device {
//     type VerifyingKey = Ed25519VerifyingKey;
// }

/// Verifies protocol messages by their computing its protocol-specific prehashed digest.
impl Verifier<DeviceSignature> for Device {
    fn verify(&self, msg: &[u8], signature: &DeviceSignature) -> Result<(), SignatureError> {
        let msg_digest = Self::protocol_message_digest::<Sha512>(msg);
        Ok(self.verify_digest(msg_digest, signature)?)
    }
}

/// A default [`Device`] with a secret key of all zeros.
impl Default for Device {
    fn default() -> Self {
        let sk = Ed25519SigningKey::from_bytes(&[0u8; 32]);
        let pk = sk.verifying_key();
        Self {
            inner: DeviceInner::Ed25519(pk),
        }
    }
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn default() {
    //     use super::*;
    //     let peer = Device::default();
    //     assert_eq!(peer.verifying_key().as_bytes(), &NULL_PEER_KEY);
    // }
}
