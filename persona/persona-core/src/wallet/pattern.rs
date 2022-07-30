use super::*;
use crate::*;

///
pub type PatternList<const S: usize> = [Pattern; S];

/// Invoking an instruction on behalf of this wallet requires that the
/// instruction match against this or another of the wallet's patterns. If the
/// `program_id` is this program, then this pattern can be used to override the
/// wallet's own [`Threshold`]s.
///
/// [`Threshold`]:
#[derive(Copy, Clone, core::fmt::Debug, Eq, Hash, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Pattern {
    pattern_bytes: [u8; Pattern::PAT_BYTE_SIZE],
    program_id: Pubkey,
    offset: u16,
    threshold: Threshold,
}

impl Pattern {
    ///
    pub const PAT_BYTE_SIZE: usize = 256;
    pub const BYTE_SIZE: usize =
        Self::PAT_BYTE_SIZE + PUBKEY_BYTE_SIZE + Self::OFFSET_BYTE_SIZE + THRESHOLD_BYTE_SIZE;

    const OFFSET_BYTE_SIZE: usize = 2;

    ///
    pub fn new(
        pattern_bytes_in: &[u8],
        offset: u16,
        program_id: &Pubkey,
        threshold: Threshold,
    ) -> Result<Self, Error> {
        if pattern_bytes_in.len() >= Self::PAT_BYTE_SIZE {
            return Err(Error::ExceedsMaximumPatternLength {
                len: pattern_bytes_in.len(),
            });
        }

        let len = pattern_bytes_in.len() as u8;
        let mut pattern_bytes = [0u8; Self::PAT_BYTE_SIZE];
        pattern_bytes[0] = len;
        pattern_bytes[1..].copy_from_slice(pattern_bytes_in);

        Ok(Self {
            pattern_bytes,
            offset,
            program_id: *program_id,
            threshold,
        })
    }

    ///
    pub fn pattern_len(&self) -> usize {
        self.pattern_bytes[0] as usize
    }

    ///
    pub fn pattern(&self) -> &[u8] {
        &self.pattern_bytes[1..self.pattern_len()]
    }
}

impl Default for Pattern {
    fn default() -> Self {
        Self {
            pattern_bytes: [0u8; Self::PAT_BYTE_SIZE],
            offset: 0,
            program_id: Default::default(),
            threshold: 0,
        }
    }
}

// ///
// #[derive(
//     Copy,
//     Clone,
//     core::fmt::Debug,
//     Eq,
//     PartialEq,
//     bytemuck::Pod,
//     bytemuck::Zeroable,
//     // borsh::BorshDeserialize,
//     // borsh::BorshSchema,
//     // borsh::BorshSerialize,
// )]
// #[repr(transparent)]
// pub struct PatternList<const S: usize = { PatternList::<0>::DEFAULT_LEN }>([Pattern; S]);

// // pub type DefaultPatternList = PatternList<{ PatternList::DEFAULT_SIZE }>;

// impl<const S: usize> PatternList<S> {
//     pub const DEFAULT_LEN: usize = 256;
//     pub const BYTE_SIZE: usize = 4 + (S * Pattern::BYTE_SIZE);

//     #[inline]
//     pub const fn len(&self) -> usize {
//         self.0.len()
//     }

//     #[inline]
//     pub fn at(&self, index: usize) -> Option<&Pattern> {
//         self.0.get(index)
//     }

//     // #[inline]
//     // pub fn replace
// }

// impl<const S: usize> AsRef<[Pattern]> for PatternList<S> {
//     fn as_ref(&self) -> &[Pattern] {
//         &self.0
//     }
// }

// impl<const S: usize> Default for PatternList<S> {
//     fn default() -> Self {
//         Self([Pattern::default(); S])
//     }
// }
