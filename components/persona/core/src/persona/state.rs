use crate::{util, Device, Error, SignedOperation};
use borsh::{BorshDeserialize, BorshSerialize};
use cid::Cid;

/// Each device associated with a [`Persona`] has a configurable weight that
/// governs its share in signing for persona state change operations.
pub type Weight = u8;

/// The state of a [`Persona`]. All operations require signatures from devices
/// whose weights exceed `low_weight`, while operations that change the
/// [`Keyset`] additionally require exceeding the `high_weight`.
#[derive(Clone, Debug, Default, PartialEq, Eq, BorshDeserialize, BorshSerialize)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct State {
    devices: Vec<(Device, Weight)>,
    #[borsh(
        deserialize_with = "util::cid::deserialize_cid",
        serialize_with = "util::cid::serialize_cid"
    )]
    metadata: Cid,
    seqno: u64,
}

impl State {
    // ///
    // #[inline]
    // pub fn device_by_idx(&self, idx: usize) -> Option<(&Device, Weight)> {
    //     self.devices.get(idx).map(|(k, w)| (k, *w))
    // }
}

// state validation and application
impl State {
    pub fn validate(&self, op: SignedOperation) -> Result<(), Error> {
        todo!()
    }

    pub fn apply_genesis(op: SignedOperation) -> Result<Self, Error> {
        todo!()
    }

    pub fn apply(&mut self, op: SignedOperation) -> Result<(), Error> {
        todo!()
    }
}

// impl<const SIZE: usize> Default for State<SIZE> {
//     #[inline]
//     fn default() -> Self {
//         Self {
//             devices: [Default::default(); SIZE],
//             weights: [Default::default(); SIZE],
//             high_threshold: 0,
//             low_threshold: 0,
//         }
//     }
// }
