use crate::{
    maybestd::{io, string::ToString, vec::Vec},
    util, Error,
};
use borsh::{BorshDeserialize, BorshSerialize};
use digest::Digest;
use ed25519_dalek::{Signature as Ed25519Signature, VerifyingKey};
use sha2::{Sha256, Sha512};
use signature::{DigestSigner, DigestVerifier, Error as SignatureError, Verifier};

///
pub type DeviceKey = VerifyingKey;

///
#[derive(Copy, Clone, Debug, Eq, PartialEq, BorshDeserialize, BorshSerialize)]
#[non_exhaustive]
pub enum Device {
    Ed25519(
        #[borsh(
            deserialize_with = "util::ed25519::deserialize_device_key",
            serialize_with = "util::ed25519::serialize_device_key"
        )]
        DeviceKey,
    ),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, BorshDeserialize, BorshSerialize)]
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

    /// Initializes a new [`Device`] and it's [`MerkleLog`].
    pub fn init<Si>(pk: DeviceKey, signer: &Si) -> Result<Self, Error>
    where
        Si: DigestSigner<Sha512, DeviceSignature>,
    {
        let entry = {
            let entry = Self::protocol_message_digest(pk.as_bytes());
            // assert that the signer is the owner of the public key being initialized
            let DeviceSignature::Ed25519(sig) = signer
                .try_sign_digest(entry.clone())
                .map_err(Error::SignatureError)?;
            pk.verify_digest(entry, &sig)
                .map_err(Error::SignatureError)?;
            sig
        };

        Ok(Self::Ed25519(pk))
    }

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

    // pub fn public_key(&self) -> &DeviceKey {
    //     &self.pk
    // }

    // /// Determines if this is a "null" peer, i.e. the default peer.
    // pub fn is_null_peer(&self) -> bool {
    //     self.pk.as_bytes() == &NULL_PEER_KEY
    // }

    fn protocol_message_digest(entry: impl AsRef<[u8]>) -> Sha512 {
        Sha512::new_with_prefix(Self::SIGNING_CONTEXT).chain_update(entry)
    }
}

// impl AsRef<DeviceKey> for Device {
//     fn as_ref(&self) -> &DeviceKey {
//         self.public_key()
//     }
// }

// impl KeypairRef for Device {
//     type VerifyingKey = DeviceKey;
// }

impl Verifier<DeviceSignature> for Device {
    fn verify(&self, msg: &[u8], signature: &DeviceSignature) -> Result<(), SignatureError> {
        let msg_digest = Self::protocol_message_digest(msg);
        match (self, signature) {
            (Self::Ed25519(pk), DeviceSignature::Ed25519(sig)) => pk.verify_digest(msg_digest, sig),
        }
    }
}

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
