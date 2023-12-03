use super::{
    super::{Group, Member, Persona},
    GenericOperation,
};
use crate::{proof::Operation as IOperation, util::Sha256Digest, Error};

/// ...
pub const DOMAIN_SEPARATOR: &[u8] = b"datalove::persona::sign";

///
pub type SignPayload = [u8; 64];

/// The `sign` operation, which produces a signature on behalf of a [`Persona`].
///
///
pub type Sign = GenericOperation<SignPayload>;

impl IOperation<Group, Persona> for Sign {
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
