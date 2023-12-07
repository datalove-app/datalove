use super::{Did, Weight};
use crate::{
    device::{Device, DeviceSignature},
    maybestd::{cmp, io, vec::Vec},
    proof::{ProverState, VerifierState},
    util::risc0::{Sha256Digest, TypedJournal},
    Error,
};
use borsh::{BorshDeserialize, BorshSerialize};
use signature::Verifier;

/// Publicly committed state of the persona.
#[derive(Clone, Debug, Default, Eq, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Persona {
    //// The DID of the persona.
    pub(super) did: Did,

    /// Digest of ancillary persona-related metadata to be snapshotted.
    pub(super) metadata: Sha256Digest,

    /// Digest of the proof-specific message (e.g. signature payload).
    pub(super) msg: Sha256Digest,

    /// The sequence number (ie. age by number of proofs generated).
    pub(super) seqno: u32,
}

impl Persona {
    pub const DEFAULT: Self = Self {
        did: Did::ZERO,
        msg: Sha256Digest::ZERO,
        metadata: Sha256Digest::ZERO,
        seqno: 0,
    };
}

impl VerifierState for Persona {}

impl PartialOrd for Persona {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.did
            .eq(&other.did)
            .then(|| self.seqno.cmp(&other.seqno))
    }
}

///
pub type PersonaSignature = TypedJournal<(Sha256Digest, Persona)>;

// pub struct PersonaState(TypedJournal<(Sha256Digest, Persona)>);

//
// Group
//

/// Private state managed by and known only to the [`Persona`] and its [`Member`]s.
#[derive(Clone, Debug, Default, Eq, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Group {
    // TODO: replace with merkle tree
    members: Vec<Member>,
}

impl ProverState for Group {}

impl Group {
    pub const DEFAULT: Self = Self {
        members: Vec::new(),
    };

    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }

    pub fn len(&self) -> usize {
        self.members.len()
    }

    pub fn weight(&self) -> Weight {
        self.members.iter().map(|m| m.weight()).sum()
    }

    /// Verifies a group signature against the group's state.
    pub fn verify(&self, msg: &[u8], sig: &GroupSignature) -> Result<(), Error> {
        // assert participant count
        let num_participants = sig.len();
        if !(num_participants > 0 && num_participants <= self.len()) {
            return Err(signature::Error::new().into());
        }

        let mut weight = 0;

        // verify each member sig
        // TODO: group and batch device sig verifies
        for (member_idx, member_sig) in sig.idx_iter() {
            let member = &self.members[member_idx];
            member.verify(msg, &member_sig)?;

            weight += member.weight();
        }

        // assert weight
        self.verify_weight(weight)?;

        Ok(())
    }

    /// TODO:
    fn verify_weight(&self, weight: Weight) -> Result<(), Error> {
        if weight < self.weight() / 2 {
            return Err(signature::Error::new().into());
        }

        Ok(())
    }
}

///
#[derive(Clone, Debug, Default, Eq, PartialEq, BorshSerialize)]
pub struct GroupSignature {
    indices: u32,
    signatures: Vec<MemberSignature>,
}

impl GroupSignature {
    pub fn len(&self) -> usize {
        self.signatures.len()
    }

    /// Returns an iterator over the member's group index and its signature.
    pub fn idx_iter(&self) -> impl Iterator<Item = (usize, &MemberSignature)> {
        (0..32u32)
            .filter_map(|idx| (self.indices & (1 << idx) != 0).then(|| idx as usize))
            .zip(self.signatures.iter())
    }
}

impl BorshDeserialize for GroupSignature {
    fn deserialize_reader<R: io::Read>(reader: &mut R) -> Result<Self, io::Error> {
        let indices = u32::deserialize_reader(reader)?;
        let signatures = Vec::deserialize_reader(reader)?;
        if indices.count_ones() as usize != signatures.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid signature count",
            ));
        }

        Ok(Self {
            indices,
            signatures,
        })
    }
}

// impl VerifierState<(usize, MemberSignature)> for Group {
//     fn verify(
//         &self,
//         msg: &[u8],
//         (idx, signature): &(usize, MemberSignature),
//     ) -> Result<(), signature::Error> {
//         match signature {
//             MemberSignature::Device(sig) => todo!(),
//             MemberSignature::Persona(persona) => todo!(),
//         }
//     }
// }

// impl VerifierState<GroupSignature> for Group {
//     fn verify(&self, msg: &[u8], signature: &GroupSignature) -> Result<(), signature::Error> {
//         todo!()
//     }
// }

//
// Member
//

/// A [`Persona`] is managed by members, each of which is either a device or another persona.
#[derive(Clone, Debug, Eq, PartialEq, BorshDeserialize, BorshSerialize)]
#[borsh(use_discriminant = true)]
#[non_exhaustive]
#[repr(align(4))]
pub enum Member {
    Device(MemberInner<Device>),
    Persona(MemberInner<Persona>),
}

impl Member {
    pub const fn weight(&self) -> Weight {
        match self {
            Self::Device(member) => member.weight,
            Self::Persona(member) => member.weight,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct MemberInner<T> {
    weight: Weight,
    inner: T,
}

impl Member {
    pub fn verify(&self, msg: &[u8], signature: &MemberSignature) -> Result<(), Error> {
        match (self, signature) {
            (Self::Device(member), MemberSignature::Device(sig)) => {
                member.inner.verify(msg, sig)?
            }
            (Self::Persona(member), MemberSignature::Persona(sig)) => {
                Self::verify_persona_signature(&member.inner, msg, sig)?
            }
            _ => Err(signature::Error::new())?,
        };

        Ok(())
    }

    /// Verifies a [`PersonaSignature`].
    ///
    /// Persona proofs double as signatures, and are verified upon deserialization,
    /// so this just asserts that the proof belongs to this member and signs the same message.
    fn verify_persona_signature(
        member: &Persona,
        msg: &[u8],
        signature: &PersonaSignature,
    ) -> Result<(), Error> {
        let sig = &signature.as_inner().1;

        if sig.did != member.did {
            return Err(signature::Error::new())?;
        }

        if AsRef::<[u8]>::as_ref(&sig.msg) != msg {
            return Err(signature::Error::new())?;
        }

        Ok(())
    }
}

/// A signature produced by a member of a [`Persona`].
#[derive(Clone, Debug, Eq, PartialEq, BorshDeserialize, BorshSerialize)]
#[borsh(use_discriminant = true)]
#[non_exhaustive]
#[repr(align(4))]
pub enum MemberSignature {
    Device(DeviceSignature),
    Persona(TypedJournal<(Sha256Digest, Persona)>),
}
