use super::{Float, Int};
use crate::{base::Base, cid::CID};

// TODO: ideas:
//  - implement Serialize for Stream of Tokens
//  - implement

///
#[derive(Clone, Debug, From, PartialEq)]
pub enum Token<'a> {
    ///
    Null,

    ///
    Bool(bool),

    ///
    Integer(Int),

    ///
    Float(Float),

    ///
    Str(&'a str),

    /// Raw bytes.
    Bytes(&'a [u8]),

    /// [`multibase`]-encoded `str`.
    ByteStr(&'a str),

    ///
    List(Option<usize>),

    ///
    ListEnd,

    ///
    Map(Option<usize>),

    ///
    MapEnd,

    ///
    Link(CID),
    // RawValue?

    // TODO: a link to another `Dag`, possibly of another format
    // LinkedData(Option<Prefix>),

    // TODO:
    // LinkedDataEnd,
}
