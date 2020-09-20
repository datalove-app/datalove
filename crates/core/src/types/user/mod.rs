//!
//! ```text
//! schema!(type DeviceLink &Device);
//!
//! advanced!(type Devices ...Device... representation
//!     Group());
//!     // b/c this is CRDT+Set, it has set mutations, meaning
//!         // internal representations keep around unapplied mutations
//!         // by default, all ops are applied (if valid)
//!     // b/c this is a Signer(all)
//!         // it requires a signature from all members
//!         // e.g. Signer(m_of_n(m=3,n=5))
//!     // b/c this is a Signer and a Set, it is a Group
//!
//! advanced!(type Guardians ...Device + User... representation
//!     ...);
//!
//! advanced!(type User struct {
//!     vclock: [{&Device: u64}]    // clock of clocks?
//!     devices: Devices,
//!     guardians: Guardians
//! } representation ...);
//! ```
//!
//! must be able to prove (generally):
//!     - that it's either the initial doc or a derived doc (hash)
//!     - if initial
//!         - (hash) that the DID of the doc is the CID
//!         - (parsing) that the payload contains the appropriate struct members
//!         - (hash) that the payload
//!         - (signature) that the payload
//!     - if derived
//! must be able to prove (directly):
//!     - "any state transition" about the user didoc:
//!         - MUST INCREMENT REV COUNT AND LINK TO PREVIOUS DIDOC, but also:
//!         - replace the profile hash (signed by threshold of guardians)
//!         - rotate the signing keys (signed by threshold of guardians)
//!         - freeze the account, preventing any future txn except for
//!             unfreezing (signed by only one guardian key)
//!         - unfreeze + either add or remove guardian (signed by threshold
//!             guardian keys)
//!             - adds a key (signed by threshold of guardian keys)
//!             - deletes a key (signed by a threshold of guardian keys)
//!         - any did subscription increments are valid
//! full proving pipeline - given an object's state:
//!     gets the listed user/group info (for simplicity, assuming single writer):
//!         - the writer's did
//!         - its writer's didoc hash
//!         - ? its writer's listed vclock
//!     next, fetches:
//!         - the didoc's original tail
//!         - (if you can find it) the didoc's HEAD
//!             - verifies the HEAD is directly valid (see above)
//!             - ? verifies the HEAD includes listed and tail?
//!     then verifies the listed didoc:
//!         - first, that the didoc is directly valid (see above)
//!         - then, that the object was signed by that didoc's currently-listed key
//!
//!

use crate::dev::*;

///
pub struct DID(Did);

#[cfg(test)]
mod test {

    #[test]
    fn it_works() -> Result<(), ()> {
        println!("running test");
        assert_eq!(true, true);
        Ok(())
    }
}

// mod circuit;

// use super::device::{Device, Devices};
// use async_trait::async_trait;
// use pairing::bn256::Fr;
// use std::collections::HashMap;

//#[async_trait]
//pub trait UserStorage {
//    async fn create_genesis_user() -> Result<(), ()> {
//        Ok(())
//    }
//}

// // has a DID
// pub struct UserDoc {
//     //
//     rev_count: u32,

//     // link to the previous UserDoc
//     //    prev_hash: Link<UserDoc>,

//     // current root hash of the user key tree (after applying diff_tree to init state)
//     root_hash: Fr,

//     // the "public data" required to verify the snark;
//     // w/o it, we cant verify that this did was of valid provenance
//     //    diff_tree: Link<DiffTree>,

//     // snark that proves that the did hash (from implied init state) + diff_tree ==> root_hash
//     snark: String,
//     // links to other UserDoc objects, by did and rev_count
//     // w/o continuous inclusion of this in other user's timers, we cant verify
//     // that this did was not forked at some point
//     //    timers: HashMap<(), (String, u64, Link<UserDoc>)>,

//     //
//     //    social_proofs: Vec<Link<SocialProof>>
// }

// impl UserDoc {}
