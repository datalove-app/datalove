use super::{Did, Persona, Weight};
use crate::{
    device::DeviceSignature, maybestd::vec::Vec, util::risc0::TypedJournal, Device, Error,
};
use borsh::{BorshDeserialize, BorshSerialize};
use signature::Verifier;

/// A [`Persona`] is managed by members, each of which is either a device or another persona.
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
#[borsh(use_discriminant = true)]
#[non_exhaustive]
#[repr(align(4))]
pub enum Member {
    Device(MemberInner<Device>),
    Persona(MemberInner<Persona>),
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct MemberInner<T> {
    weight: Weight,
    state: T,
}

/// A signature produced by a member of a [`Persona`].
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
#[borsh(use_discriminant = true)]
#[non_exhaustive]
#[repr(align(4))]
pub enum MemberSignature {
    Device(DeviceSignature),
    Persona(TypedJournal<Persona>),
}

/// Private state managed by and known only to the [`Persona`] and its [`Member`]s.
#[derive(Clone, Debug, Default, BorshDeserialize, BorshSerialize)]
pub struct Group {
    members: Vec<Member>,
}

///
#[derive(Clone, Debug, Default, BorshDeserialize, BorshSerialize)]
pub struct GroupSignature {
    signatures: Vec<MemberSignature>,
}

impl Verifier<MemberSignature> for Group {
    fn verify(&self, msg: &[u8], signature: &MemberSignature) -> Result<(), signature::Error> {
        match signature {
            MemberSignature::Device(sig) => todo!(),
            MemberSignature::Persona(persona) => todo!(),
        }
    }
}

impl Verifier<GroupSignature> for Group {
    fn verify(&self, msg: &[u8], signature: &GroupSignature) -> Result<(), signature::Error> {
        todo!()
    }
}
