//!
//! similar to a bloom clock, but clock itself is a sparse merkle tree
//! - allows the clock to be represented by a hash rather than a vector of ints
//! -
//!
//! - incrementing the clock:
//!     - hash of the event (of len 256) is the index (aka Left/Right path) in tree (of size 2^256) that is set to 1
//!     - produces a path proof from the new (incremented?) leaf to the new root
//! - merging two clocks
//!     - given two sets of hashed events
//!
