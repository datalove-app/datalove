mod ops;
mod state;

pub use ops::*;
pub use state::*;

use crate::{util, Error};
use borsh::{io, BorshDeserialize, BorshSerialize};
use risc0_zkvm::Receipt;

///
#[derive(Debug, PartialEq)]
pub struct Persona {
    // status: WalletStatus,
    state: State,
    prev: Option<Receipt>,
    // did: String,
}

impl Persona {
    ///
    pub const MAX_DEVICES: usize = 32;

    // /// Generates random owners.
    // #[doc(hidden)]
    // pub fn random_owners(size: usize, threshold: Weight) -> Owners {
    //     assert!(size <= Self::MAX_OWNERS);

    //     let mut owners: Owners = Default::default();
    //     for i in 0..size {
    //         owners[i] = (Pubkey::new_unique(), threshold);
    //     }
    //     owners
    // }
}

impl Persona {
    // #[inline]
    // pub fn is_active(&self) -> bool {
    //     self.state == WalletStatus::Active
    // }

    // #[inline]
    // pub fn is_frozen(&self) -> bool {
    //     self.state == WalletStatus::Frozen
    // }
}

impl BorshSerialize for Persona {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        self.state.serialize(writer)?;
        match &self.prev {
            None => u8::from(0).serialize(writer)?,
            Some(receipt) => {
                u8::from(1).serialize(writer)?;
                util::risc0::serialize_receipt(receipt, writer)?
            }
        };

        Ok(())
    }
}

impl BorshDeserialize for Persona {
    fn deserialize_reader<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let state = State::deserialize_reader(reader)?;
        let prev = {
            match u8::deserialize_reader(reader)? {
                0 => None,
                1 => Some(util::risc0::deserialize_receipt(reader)?),
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "invalid Option<Receipt> flag",
                    ))
                }
            }
        };

        Ok(Self { state, prev })
    }
}

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
