mod group;
mod ops;

pub use group::{Group, GroupSignature, Member, MemberSignature, Persona};
pub use ops::SignedOperation;

use crate::util::Sha256Digest;

///
pub type Weight = u8;

///
pub type Threshold = u16;

///
pub type Did = Sha256Digest;

///
#[cfg(target_os = "zkvm")]
pub mod guest {
    use super::*;
    use crate::{maybestd::io, zksm::StateMachine, Error};

    pub fn exec(
        mut stdin: impl io::Read,
        mut stdout: impl io::Write,
        mut journal: impl io::Write,
        // mut pause: impl FnMut() -> bool,
    ) -> Result<(), Error> {
        StateMachine::<Group, Persona>::run_io::<SignedOperation>(
            &mut stdin,
            &mut stdout,
            &mut journal,
        )
    }
}
