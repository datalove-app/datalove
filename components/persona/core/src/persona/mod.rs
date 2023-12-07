mod group;
mod ops;

pub use group::{Group, GroupSignature, Member, MemberSignature, Persona};
pub use ops::Operation;

use crate::{maybestd::io, proof::StateMachine, util::Sha256Digest, Error};

///
pub type Weight = u8;

///
pub type Did = Sha256Digest;

///
pub fn exec(
    mut stdin: impl io::Read,
    mut stdout: impl io::Write,
    mut journal: impl io::Write,
) -> Result<(), Error> {
    StateMachine::<Group, Persona>::exec::<Operation>(&mut stdin, &mut stdout, &mut journal)
}
