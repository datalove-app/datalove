use super::*;
use crate::*;
use core::cmp::Ordering;

///
#[derive(
    Copy,
    Clone,
    core::fmt::Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    bytemuck::Pod,
    bytemuck::Zeroable,
)]
#[repr(C)]
pub struct Signatory(Pubkey, Threshold);

impl Signatory {
    pub const BYTE_SIZE: usize = PUBKEY_BYTE_SIZE + THRESHOLD_BYTE_SIZE;

    #[cfg(not(feature = "std"))]
    pub const ZEROES: Self = Self(pubkey::ZEROES, 0);

    pub fn key(&self) -> Pubkey {
        self.0
    }

    pub fn threshold(&self) -> Threshold {
        self.1
    }
}

impl From<Signatory> for (Pubkey, Threshold) {
    fn from(signatory: Signatory) -> Self {
        (signatory.0, signatory.1)
    }
}

impl PartialOrd for Signatory {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.threshold().partial_cmp(&other.threshold())
    }
}

macro_rules! gen_signatories {
    ($name:ident => $size:expr) => {
        ///
        #[derive(
            Copy,
            Clone,
            core::fmt::Debug,
            Eq,
            Hash,
            PartialEq,
            bytemuck::Pod,
            bytemuck::Zeroable,
            // borsh::BorshDeserialize,
            // borsh::BorshSchema,
            // borsh::BorshSerialize,
        )]
        #[repr(C)]
        pub struct $name {
            keys: [Pubkey; $name::LEN],
            thresholds: [Threshold; $name::LEN],
        }

        static_assertions::const_assert!($name::MAX_LEN >= $size);

        impl $name {
            const MAX_LEN: usize = 32;
            pub const LEN: usize = $size;
            pub const BYTE_SIZE: usize = 4 + (Self::LEN * Signatory::BYTE_SIZE);
            pub const ZEROES: Self = Self {
                keys: [pubkey::ZEROES; Self::LEN],
                thresholds: [0; Self::LEN],
            };

            /// Generate a compact signatories struct from a slice of
            /// [`Signatory`]s.
            pub fn try_from(signatories: &[Signatory]) -> Result<Self, Error> {
                if !(signatories.len() > 0 && signatories.len() <= Self::LEN) {
                    Err(Error::invalid_guardian_count(signatories.len(), Self::LEN))
                } else {
                    let mut keys = [pubkey::ZEROES; Self::LEN];
                    let mut thresholds = [0; Self::LEN];

                    for (i, (k, t)) in signatories.iter().map(|s| (*s).into()).enumerate() {
                        keys[i] = k;
                        thresholds[i] = t;
                    }

                    Ok(Self { keys, thresholds })
                }
            }

            /// Quantity of set signatories.
            pub fn len(&self) -> usize {
                self.keys.iter().filter(|k| *k != &pubkey::ZEROES).count()
            }

            /// Fetches the [`Signatory`] at a given index.
            ///
            /// [`Signatory`]: crate::Signatory
            pub fn at(&self, index: usize) -> Option<Signatory> {
                match (
                    self.keys.get(index).copied(),
                    self.thresholds.get(index).copied(),
                ) {
                    (Some(key), Some(threshold)) => Some(Signatory(key, threshold)),
                    _ => None,
                }
            }

            pub fn is_signatory(&self, key: &Pubkey) -> bool {
                self.keys.iter().any(|s| &s == &key)
            }

            /// Fetches the [`Threshold`] for the given [`Signatory`]'s public
            /// key.
            pub fn threshold_for(&self, signatory: &Pubkey) -> Option<&Threshold> {
                if let Some((idx, _)) = (&self.keys)
                    .iter()
                    .enumerate()
                    .find(|(_, t)| t == &signatory)
                {
                    Some(&self.thresholds[idx])
                } else {
                    None
                }
            }

            /// Returns the max reachable [`Threshold`] for the entire set of
            /// [`Signatory`]s.
            pub fn ceiling(&self) -> Option<Threshold> {
                checked_sum(self.thresholds.iter())
            }

            /// Returns the max reachable [`Threshold`] for the provided set of
            /// public keys.
            pub fn ceiling_for(&self, signatories: &[Pubkey]) -> Option<Threshold> {
                checked_sum(signatories.iter().filter_map(|s| self.threshold_for(s)))
            }

            #[cfg(feature = "std")]
            /// Generates random [`Signatory`]s.
            pub fn random(threshold: Threshold) -> Self {
                let mut keys = [pubkey::ZEROES; Self::LEN];
                let mut thresholds = [threshold; Self::LEN];
                for i in 0..Self::LEN {
                    keys[i] = Pubkey::new_unique();
                }
                Self { keys, thresholds }
            }
        }

        // impl AsRef<[Signatory]> for $name {
        //     fn as_ref(&self) -> &[Signatory] {
        //         &self.0
        //     }
        // }

        impl Default for $name {
            fn default() -> Self {
                Self::ZEROES
            }
        }

        impl PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.ceiling()
                    .and_then(|s| other.ceiling().map(|o| (s, o)))
                    .and_then(|(ref s, ref o)| PartialOrd::partial_cmp(s, o))
            }
        }
    };
}

gen_signatories!(PrimarySignatories => 4);
gen_signatories!(FullSignatories => 32);

fn checked_sum<'a>(iter: impl Iterator<Item = &'a Threshold>) -> Option<Threshold> {
    iter.fold(Some(0), |sum, i| sum.and_then(|sum| sum.checked_add(*i)))
}
