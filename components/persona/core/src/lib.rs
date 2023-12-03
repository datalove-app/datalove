//! datalove-persona-core
//!
//! Each persona is managed by an evolving set of devices, each of which has a
//! keypair used to secure network communications and sign operations on behalf of the persona.
//!
//! ## Components:
//!
//! ### Device:
//! An [`ed25519`] keypair and a [`MerkleLog`] of signed messages, the first of which
//! is the signed public key itself.
//!
//! ## Stages:
//!
//! ### Genesis:
//! A device
//!
//! ### Continued operation:
//!

#![cfg_attr(not(feature = "std"), no_std)]

mod device;
mod error;
mod persona;

pub(crate) mod proof;
pub(crate) mod util;
pub(crate) mod maybestd {
    pub use borsh::__private::maybestd::*;
    pub use borsh::io;
    pub use core::{marker, ops};
}

pub use borsh;
pub use error::Error;
pub use persona::*;

// #[doc(hidden)]
// pub use persona::State;

// ///
// #[derive(Debug, PartialEq)]
// pub struct Persona {
//     state: PublicState,
//     prev: Option<Receipt>,
//     // did: String,
// }

// pub enum Signature {
//     Device(ed25519_dalek::Signature),
//     Persona {
//         image_id: risc0_zkp::Digest,
//         journal: Journal,
//     }
// }

// impl Persona {
//     // /// Generates random owners.
//     // #[doc(hidden)]
//     // pub fn random_owners(size: usize, threshold: Weight) -> Owners {
//     //     assert!(size <= Self::MAX_OWNERS);

//     //     let mut owners: Owners = Default::default();
//     //     for i in 0..size {
//     //         owners[i] = (Pubkey::new_unique(), threshold);
//     //     }
//     //     owners
//     // }
// }

// impl Persona {
//     // #[inline]
//     // pub fn is_active(&self) -> bool {
//     //     self.state == WalletStatus::Active
//     // }

//     // #[inline]
//     // pub fn is_frozen(&self) -> bool {
//     //     self.state == WalletStatus::Frozen
//     // }
// }

// impl BorshSerialize for Persona {
//     fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
//         self.state.serialize(writer)?;
//         match &self.prev {
//             None => u8::from(0).serialize(writer)?,
//             Some(receipt) => {
//                 u8::from(1).serialize(writer)?;
//                 util::risc0::serialize_receipt(receipt, writer)?
//             }
//         };

//         Ok(())
//     }
// }

// impl BorshDeserialize for Persona {
//     fn deserialize_reader<R: io::Read>(reader: &mut R) -> io::Result<Self> {
//         let state = State::deserialize_reader(reader)?;
//         let prev = {
//             match u8::deserialize_reader(reader)? {
//                 0 => None,
//                 1 => Some(util::risc0::deserialize_receipt(reader)?),
//                 _ => {
//                     return Err(io::Error::new(
//                         io::ErrorKind::InvalidData,
//                         "invalid Option<Receipt> flag",
//                     ))
//                 }
//             }
//         };

//         Ok(Self { state, prev })
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn invalid_wallets() {
    //     let invalid_threshold = User::new(100, 100, Default::default(), Default::default());
    //     assert!(
    //         invalid_threshold.is_err(),
    //         "wallet is invalid; this assertion should fail"
    //     );

    //     let invalid_patterns = User::new(100, 100, Default::default(), Default::default());
    //     assert!(
    //         invalid_patterns.is_err(),
    //         "wallet is invalid; this assertion should fail"
    //     );
    // }
}
