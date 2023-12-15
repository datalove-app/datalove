use super::{Did, Weight};
use crate::{
    device::{Device, DeviceSignature},
    maybestd::{
        cmp,
        collections::BTreeMap,
        io,
        vec::{IntoIter, Vec},
    },
    util::risc0::{Sha256Digest, TypedJournal},
    zksm::{ProverState, VerifierState},
    Error, Threshold,
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
    /// usually a persona operation, but could be a message to just sign
    pub(super) msg: Sha256Digest,

    // /// Digest of the group managing this persona.
    // pub(super) group: Sha256Digest,
    /// The sequence number (ie. age by number of proofs generated).
    pub(super) seqno: u32,
    // ///
    // pub(super) clock: BloomClock<4, 96, Sha256>,
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

impl PersonaSignature {
    // (Self::Persona(member), MemberSignature::Persona(sig)) => {
    //     // Persona proofs double as signatures, and are verified upon deserialization,
    //     // so this just asserts that the proof belongs to this member and signs the same message.

    //     let persona_sig = &sig.payload.as_inner().1;

    //     if persona_sig.did != member.payload.did {
    //         Err(Error::InvalidSignatureError(
    //             "signature DID does not match member DID",
    //         ))?;
    //     }

    //     if &persona_sig.msg != &member_op_digest {
    //         Err(Error::InvalidSignatureError(
    //             "signature message does not apply to operation",
    //         ))?;
    //     }
    // }
    // _ => Err(Error::InvalidSignatureError(
    //     "signature / member type mismatch",
    // ))?,
}

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

    // pub const HIGH_THRESHOLD: Threshold = u16::MAX >> 2; // 16383
    // pub const MID_THRESHOLD: Threshold = u16::MAX >> 4; // 4095
    // pub const LOW_THRESHOLD: Threshold = u16::MAX >> 8; // 255

    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }

    pub fn len(&self) -> usize {
        self.members.len()
    }

    /// Returns the weight of the group.
    /// TODO: refactor this mechanism
    // pub fn group_weight(&self) -> Threshold {
    //     self.weights().fold(0u16, |a, b| a + b as Threshold)
    // }

    // pub fn weights(&self) -> impl Iterator<Item = Weight> + '_ {
    //     self.members.iter().map(|m| m.weight())
    // }

    // pub fn devices(&self) -> impl Iterator<Item = Member> + '_ {
    //     self.members.iter().filter_map(|m| match m {
    //         Member::Device(d) => Some(m),
    //         _ => None,
    //     })
    // }

    ///
    /// TODO: verify sig against group
    pub fn add_signature(
        &self,
        idx: u8,
        member_sig: MemberSignature,
        group_sig: GroupSignature,
    ) -> Result<GroupSignature, Error> {
        // prune duplicate signer indices
        let mut signers = self
            .signer_iter(group_sig)
            .map(|res| res.map(|signer| (signer.0, signer)))
            .collect::<Result<BTreeMap<u8, _>, _>>()?;

        // TODO: verify each current member_sig against stored member
        let member = self
            .members
            .get(idx as usize)
            .ok_or_else(|| Error::InvalidSignatureError("signature member index out of bounds"))?;

        // add signer to group_sig at it appropriate index, erroring it already exists
        signers
            .insert(idx, (idx, member, member_sig))
            .map(|_| {
                Err(Error::InvalidSignatureError(
                    "signature member index already signed",
                ))
            })
            .transpose()?;

        // aggregate indices and signatures
        let indices = signers.values().fold(0u32, |i, (idx, _, _)| i | (1 << idx));
        let signatures = signers.into_values().map(|(_, _, sig)| sig).collect();
        Ok(GroupSignature {
            indices,
            signatures,
        })
    }

    /// Verifies a group signature against the group's state.
    pub fn verify_signature(
        &self,
        op_digest: &Sha256Digest,
        sig: &GroupSignature,
    ) -> Result<Threshold, Error> {
        // assert participant count
        let num_participants = sig.len();
        if !(num_participants > 0 && num_participants <= self.len()) {
            Err(Error::InvalidSignatureError(
                "signature must have non-zero number of participants",
            ))?;
        }

        let mut sig_weight: Threshold = 0;

        // verify each member sig
        // TODO: group and batch device sig verifies
        while let Some((_, member, sig)) = self.signer_ref_iter(sig).next().transpose()? {
            member.verify_signature(op_digest, &sig)?;
            sig_weight += member.weight() as Threshold;
        }

        Ok(sig_weight)
    }

    fn signer_iter(
        &self,
        sig: GroupSignature,
    ) -> impl Iterator<Item = Result<(u8, &Member, MemberSignature), Error>> + '_ {
        sig.into_iter().map(move |(idx, sig)| {
            let member = self.members.get(idx as usize).ok_or_else(|| {
                Error::InvalidSignatureError("signature member index out of bounds")
            })?;
            Ok((idx, member, sig))
        })
    }

    fn signer_ref_iter<'a: 'b, 'b>(
        &'a self,
        sig: &'b GroupSignature,
    ) -> impl Iterator<Item = Result<(u8, &'a Member, &'b MemberSignature), Error>> + 'b {
        sig.iter().map(move |(idx, sig)| {
            let member = self.members.get(idx as usize).ok_or_else(|| {
                Error::InvalidSignatureError("signature member index out of bounds")
            })?;
            Ok((idx, member, sig))
        })
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

    fn indices_iter(&self) -> impl Iterator<Item = u8> {
        let indices = self.indices;
        (0..32u8)
            .into_iter()
            .filter_map(move |idx| ((indices & (1 << idx)) != 0).then(|| idx))
    }

    /// Returns an iterator over the member's group index and its signature.
    fn iter(&self) -> impl Iterator<Item = (u8, &MemberSignature)> {
        self.indices_iter().zip(self.signatures.iter())
    }

    fn into_iter(self) -> impl Iterator<Item = (u8, MemberSignature)> {
        self.indices_iter().zip(self.signatures.into_iter())
    }
}

impl BorshDeserialize for GroupSignature {
    fn deserialize_reader<R: io::Read>(reader: &mut R) -> Result<Self, io::Error> {
        let indices = u32::deserialize_reader(reader)?;
        let signatures = Vec::deserialize_reader(reader)?;
        if indices.count_ones() as usize != signatures.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "mismatch between num indices and signatures",
            ));
        }

        Ok(Self {
            indices,
            signatures,
        })
    }
}

//
// Member
//

/// A [`Persona`] is managed by members, each of which is either a device or another persona.
#[derive(Clone, Debug, Eq, PartialEq, BorshDeserialize, BorshSerialize)]
#[borsh(use_discriminant = true)]
#[non_exhaustive]
// #[repr(align(4))]
pub enum Member {
    Device(MemberInner<Device>),
    // Persona(MemberInner<Persona>),
}

impl Member {
    pub fn id(&self) -> [u8; 32] {
        match self {
            Self::Device(member) => member.payload.id(),
            // Self::Persona(member) => member.payload.id(),
        }
    }

    /// Returns the weight of the member.
    pub const fn weight(&self) -> Weight {
        match self {
            Self::Device(member) => member.weight,
            // Self::Persona(member) => member.weight,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct MemberInner<T> {
    weight: Weight,
    payload: T,
}

#[derive(Clone, Debug, Eq, PartialEq, BorshDeserialize, BorshSerialize)]
#[borsh(use_discriminant = true)]
#[non_exhaustive]
pub enum MemberSignature {
    Device(MemberInner<DeviceSignature>),
    // Persona(MemberInner<PersonaSignature>),
}

impl MemberSignature {
    /// Returns the weight of the member's signature.
    pub const fn weight(&self) -> Weight {
        match self {
            Self::Device(member) => member.weight,
            // Self::Persona(member) => member.weight,
        }
    }
}

impl Member {
    pub fn verify_signature(
        &self,
        op_digest: &Sha256Digest,
        signature: &MemberSignature,
    ) -> Result<(), Error> {
        // assert signer's weight is lte member weight
        if !(signature.weight() <= self.weight()) {
            Err(Error::InvalidSignatureError(
                "signature weight exceeds member weight",
            ))?;
        }

        // create modified op digest, reflecting signer's weight for this operation
        let member_op_digest = { *op_digest ^ signature.weight() as u32 };

        match (self, signature) {
            (Self::Device(member), MemberSignature::Device(sig)) => member
                .payload
                .verify(member_op_digest.as_ref(), &sig.payload)?,
            // (Self::Persona(member), MemberSignature::Persona(sig)) => {
            //     // Persona proofs double as signatures, and are verified upon deserialization,
            //     // so this just asserts that the proof belongs to this member and signs the same message.

            //     let persona_sig = &sig.payload.as_inner().1;

            //     if persona_sig.did != member.payload.did {
            //         Err(Error::InvalidSignatureError(
            //             "signature DID does not match member DID",
            //         ))?;
            //     }

            //     if &persona_sig.msg != &member_op_digest {
            //         Err(Error::InvalidSignatureError(
            //             "signature message does not apply to operation",
            //         ))?;
            //     }
            // }
            // _ => Err(Error::InvalidSignatureError(
            //     "signature / member type mismatch",
            // ))?,
        };

        Ok(())
    }
}
