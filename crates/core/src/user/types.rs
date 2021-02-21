use crate::dev::*;
use serde::{Deserializer, Serializer};

///
#[derive(Debug)]
pub struct Did(PeerId);

impl Serialize for Did {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        unimplemented!()
    }
}

impl<'de> Deserialize<'de> for Did {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        unimplemented!()
    }
}

lazy_static! {
    pub static ref GENESIS: UserRecord = { unimplemented!() };
}

///
/// NOTE:
/// must be able to prove (generally):
///     - that it's either the initial doc or a derived doc (hash)
///
///     - if initial
///         - (hash) the DID of the doc is the CID
///         - (parsing) that the payload contains the appropriate struct members
///         - (hash) that the payload
///         - (signature) that the payload
///     - if derived
///         - ...
///
/// must be able to prove (directly):
///     - "any state transition" about the user didoc:
///         - MUST INCREMENT REV COUNT AND LINK TO PREVIOUS DIDOC, but also:
///         - replace the profile hash (signed by threshold of guardians)
///         - rotate the signing keys (signed by threshold of guardians)
///         - freeze the account, preventing any future txn except for
///             unfreezing (signed by only one guardian key)
///         - unfreeze + either add or remove guardian (signed by threshold
///             guardian keys)
///             - adds a key (signed by threshold of guardian keys)
///             - deletes a key (signed by a threshold of guardian keys)
///         - any did subscription increments are valid
/// full proving pipeline - given an object's state:
///     gets the listed user/group info (for simplicity, assuming single writer):
///         - the writer's did
///         - its writer's didoc hash
///         - ? its writer's listed vclock
///     next, fetches:
///         - the didoc's original tail
///         - (if you can find it) the didoc's HEAD
///             - verifies the HEAD is directly valid (see above)
///             - ? verifies the HEAD includes listed and tail?
///     then verifies the listed didoc:
///         - first, that the didoc is directly valid (see above)
///         - then, that the object was signed by that didoc's currently-listed key
///
///
#[derive(Debug)]
pub struct UserRecord {
    // /// Link to the seed `UserRecord`, which is produced from the common
    // /// genesis `UserRecord`.
    // seed: Link<UserRecord>,

    // /// Links to the parent `UserRecord`.
    // /// [`PeerId`]:
    // // parent: Link<UserRecord>,
    // /// The current timestamp of this record.
    // clock: BloomClock<u64>,
    ///

    // // current root hash of the user key tree (after applying diff_tree to init state)
    // root_hash: Fr,

    // // the "public data" required to verify the snark;
    // // w/o it, we cant verify that this did was of valid provenance
    // //    diff_tree: Link<DiffTree>,

    // // snark that proves that the did hash (from implied init state) + diff_tree ==> root_hash
    // snark: String,
    // // links to other UserDoc objects, by did and rev_count
    // // w/o continuous inclusion of this in other user's timers, we cant verify
    // // that this did was not forked at some point
    // //    timers: HashMap<(), (String, u64, Link<UserDoc>)>,

    // //
    // //    social_proofs: Vec<Link<SocialProof>>

    /// The signature of the serialized form of the above information.
    /// TODO: how to handle? HMAC? sign-then-append? nested?
    signature: Vec<u8>,
}

impl UserRecord {}
