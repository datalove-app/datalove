mod error;
mod pattern;
mod signatory;

pub use error::Error;
pub use pattern::*;
pub use signatory::*;

pub const PUBKEY_BYTE_SIZE: usize = 32;

///
pub type Threshold = u16;
pub const THRESHOLD_BYTE_SIZE: usize = 2;

macro_rules! gen_wallet {
    ($name:ident => $signatories:ty) => {
        // #[cfg_attr(feature = "anchor_lang", anchor_lang::prelude::account)]
        // #[cfg_attr(feature = "anchor_lang", derive(Copy))]
        #[derive(Copy, Clone, Debug, Default, Eq, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
        #[repr(C)]
        pub struct $name {
            ///
            ///
            pub signatories: $signatories,

            /// The [`Threshold`] to be met by one or more signatories to
            /// augment this wallet (unless overidden by a [`Pattern`]).
            ///
            /// .. add/remove signatories
            /// .. add/remove patterns
            /// .. thaw wallet
            /// ..
            pub high_threshold: Threshold,

            /// The [`Threshold`] to be met by one or more signatories to act on
            /// this wallet's behalf
            ///
            /// .. freeze wallet
            pub low_threshold: Threshold,
        }

        impl $name {
            pub const BYTE_SIZE: usize = (2 * THRESHOLD_BYTE_SIZE) + <$signatories>::BYTE_SIZE;

            ///
            pub fn new(
                high_threshold: Threshold,
                low_threshold: Threshold,
                signatories: &[Signatory],
            ) -> Result<Self, Error> {
                let signatories = <$signatories>::try_from(signatories)?;
                Self::validate_new(high_threshold, low_threshold, &signatories)?;
                Ok(Self {
                    high_threshold,
                    low_threshold,
                    signatories,
                })
            }

            ///
            #[must_use]
            pub fn validate_new(
                high_threshold: Threshold,
                low_threshold: Threshold,
                signatories: &$signatories,
            ) -> Result<(), Error> {
                // // num signatories
                // if !(signatories.len() > 0 && signatories.len() <= <$signatories>::MAX_LEN) {
                //     return Err(Error::invalid_guardian_count(
                //         signatories.len(),
                //         <$signatories>::MAX_LEN,
                //     ));
                // }

                // thresholds
                if !(low_threshold <= high_threshold) {
                    return Err(Error::InvalidRelativeThresholds);
                }

                // minimum owner threshold
                let total_threshold = signatories
                    .ceiling()
                    .ok_or_else(|| Error::InvalidThresholdSum)?;
                if !(total_threshold >= high_threshold) {
                    return Err(Error::InsufficientGuardianThreshold);
                }

                Ok(())
            }

            ///
            #[inline]
            pub fn thresholds(&self) -> (Threshold, Threshold) {
                (self.high_threshold, self.low_threshold)
            }
        }
    };
}

gen_wallet!(FullWallet => FullSignatories);
gen_wallet!(SubWallet => PrimarySignatories);
