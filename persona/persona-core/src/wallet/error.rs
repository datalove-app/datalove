use crate::*;
use snafu::prelude::*;

#[derive(core::fmt::Debug, Snafu)]
pub enum Error {
    #[snafu(display("high threshold must be greater than low threshold"))]
    InvalidRelativeThresholds,

    #[snafu(display("total threshold sum must be between 0 and max ({})", Threshold::MAX))]
    InvalidThresholdSum,

    // #[error("high threshold must be greater than low threshold")]
    // InvalidThresholds,

    // #[error("exceeds max threshold ({})", u16::MAX)]
    // ExceedsMaxThreshold,
    #[snafu(display(
        "number of guardians must be between 0 and max/default ({max_len}), got {len}"
    ))]
    InvalidGuardianCount { len: usize, max_len: usize },

    #[snafu(display("guardian threshold sum must be greater than high threshold"))]
    InsufficientGuardianThreshold,

    #[snafu(display("{msg}"))]
    InvalidPatternList { msg: &'static str },

    #[snafu(display(
        "Pattern must be less than {} bytes; got `{len}`",
        Pattern::PAT_BYTE_SIZE,
    ))]
    ExceedsMaximumPatternLength { len: usize },

    #[snafu(display(""))]
    Unauthorized,
}

impl Error {
    pub const fn invalid_guardian_count(len: usize, max_len: usize) -> Self {
        Self::InvalidGuardianCount { len, max_len }
    }
}
