use super::{
    super::{Group, Member, Persona},
    GenericOperation,
};
use crate::{proof::Operation as IOperation, util::Sha256Digest, Error};
use borsh::{BorshDeserialize, BorshSerialize};

/// The member swap operation, which adds, removes or replaces members of the [`Persona`].
pub type Swap = GenericOperation<SwapInner>;

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct SwapInner {
    members_to_remove: Vec<Member>,
    members_to_add: Vec<Member>,
}

impl IOperation<Group, Persona> for Swap {
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
