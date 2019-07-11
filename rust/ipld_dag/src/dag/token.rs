use super::{Float, Int};
use crate::cid::CID;
use multibase::Base;
use serde_token::Token as SerdeToken;

// TODO: ideas:
//  - implement Serialize for Stream of Tokens
//  - implement

///
#[derive(From)]
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

    ///
    Bytes(&'a [u8], Option<Base>),

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
    // TODO: a link to another `Dag`, possibly of another format
    // LinkedData(Option<Prefix>),

    // TODO:
    // LinkedDataEnd,
}
