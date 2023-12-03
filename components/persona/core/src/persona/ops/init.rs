use super::{
    super::{Group, Member, Persona},
    GenericOperation,
};
use crate::{proof::Operation as IOperation, util::Sha256Digest, Error};

/// The initialization operation, that creates a new [`Persona`] from the provided [`MemberState`].
pub type Init = GenericOperation<Group>;

impl IOperation<Group, Persona> for Init {
    fn prev_outputs_digest(&self) -> &Sha256Digest {
        &self.prev_state_digest
    }

    fn validate(&self, persona: &Persona, group: &Group) -> Result<(), Error> {
        todo!()
    }

    fn apply(mut self, persona: Persona, group: Group) -> Result<Persona, Error> {
        todo!()
    }
}
