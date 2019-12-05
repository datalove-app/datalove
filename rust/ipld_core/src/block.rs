use crate::{cid::CID, dag::{Dag, RawDag}};

/**
 * TODO: ideas:
 *  - ?? possibly implement (or require) Read
 *  - ?? integrate with multihash ??
 */


///
pub trait Block {
    fn cid(&self) -> CID;

    fn data(&self) -> Read;

    fn select<S>(&self, selector: S) -> RawDag<T> where S: Selector, T: Dag;
}
