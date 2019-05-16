use self::{
    error::Error,
};
use super::atoms;
use rustler::{dynamic, Encoder, Env, types::tuple, Term, TermType};
use serde::ser::{self, Serialize};

pub mod error;
mod proxy;

// pub fn to_term<'a, T>(env: Env<'a>, value: &T) -> Result<Term<'a>> where T: Serialize {}

fn is_primitive_term(term: &Term) -> bool {
    match dynamic::get_type(*term) {
        TermType::Atom => true,
        TermType::Binary => true,
        TermType::EmptyList => true,
        TermType::Number => true,
        _ => false,
    }
}

fn is_associated_term(term: &Term) -> bool {
    match dynamic::get_type(*term) {
        TermType::List => true,
        TermType::Map => true,
        TermType::Tuple => true,
        _ => false,
    }
}

/**
 *
 */
pub struct Serializer<'a> {
    formatter: Formatter<'a>,
    output: Option<Term<'a>>,
}

impl<'a> From<Env<'a>> for Serializer<'a> {
    fn from(env: Env<'a>) -> Serializer<'a> {
        Serializer {
            formatter: Formatter{env: env, proxies: Vec::new()},
            output: None,
        }
    }
}

/**
 *
 * Proxy stack is either a single primitive, or a list of incomplete associated data types, the last one representing the deepest and current associated type being added to
 * adding initial term ---- (push)
 *  - stack.push()
 * adding next primitive ---- (add)
 *  - stack.tail() -> output.add()
 * starting next associated ---- (push)
 *  - stack.push()
 * ending next associated ---- (pop, to_term, add)
 *  - stack.pop() -> output.to_term()
 *  - stack.tail() -> output.add()
 */
impl<'a> Serializer<'a> {

}

impl<'env, 'a: 'env> ser::Serializer for &'a mut Serializer<'env> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_primitive(atoms::nil())
    }

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_primitive(v)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(self)
    }
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_primitive(v)
    }
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_primitive(v)
    }
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_primitive(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_primitive(v)
    }
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_primitive(v)
    }
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_primitive(v)
    }
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_primitive(v)
    }
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_primitive(v)
    }
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_primitive(v)
    }
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_primitive(v)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(self)
    }

    //TODO
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    // TODO
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_primitive(v)
    }

    // TODO: return Binary or OwnedBinary?
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        // use serde::ser::SerializeSeq;
        // let mut seq = self.serialize_seq(Some(v.len()))?;
        // for byte in v {
        //     seq.serialize_element(byte)?;
        // }
        // seq.end()
        Err(Error::Invalid)
    }

    // TODO
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        let compound = CompoundProxy::new_map(Some(1));
        self.formatter.start_compound(compound);
        self.serialize_str(variant);
        value.serialize(self);
        self.formatter.end_compound()?;

        Ok(())
    }

    // Now we get to the serialization of compound types.
    //
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let proxy = CompoundProxy::new_sequence(len);
        self.formatter.start_compound(proxy);
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        let proxy = CompoundProxy::new_tuple(Some(len));
        self.formatter.start_compound(proxy);
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_tuple(len)
    }

    // TODO
    // Serializes `E::T` of `enum E { T(u8, u8) }` into `%{"T" => (u8, u8)}`
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.serialize_map(Some(1))?;
        variant.serialize(self)?;
        self.serialize_tuple(len)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let proxy = CompoundProxy::new_map(len);
        self.formatter.start_compound(proxy);
        Ok(self)
    }

    // Serializes structs as plain maps (as we don't want by default to create atoms for struct names).
    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    // TODO
    // Serializes `E::S` of `enum E { S { r: u8, g: u8, b: u8 } }` into `%{"S" => %{...}}`
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.serialize_map(Some(1))?;
        variant.serialize(self)?;
        self.serialize_map(Some(len))
    }
}

// === === === === === === === === === === === === === === === === === ===
// === === === === === === === === === === === === === === === === === ===
// === === === === === === === === === === === === === === === === === ===
// === === === === === === === === === === === === === === === === === ===
// === === === === === === === === === === === === === === === === === ===
// === === === === === === === === === === === === === === === === === ===

// The following 7 impls deal with the serialization of compound types like
// sequences and maps. Serialization of such types is begun by a Serializer
// method and followed by zero or more calls to serialize individual elements of
// the compound type and one call to end the compound type.
//
// This impl is SerializeSeq so these methods are called after `serialize_seq`
// is called on the Serializer.
impl<'a> ser::SerializeSeq for &'a mut Serializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(*self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.formatter.end_compound()?;
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(*self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.formatter.end_compound()?;
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(*self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.formatter.end_compound()?;
        Ok(())
    }
}

// TODO
impl<'a> ser::SerializeTupleVariant for &'a mut Serializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(*self)
    }

    // Ends the tuple and the containing map
    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.formatter.end_compound()?;
        self.formatter.end_compound()?;
        Ok(())
    }
}

// TODO
// Some `ser::Serialize` types are not able to hold a key and value in memory at the
// same time so `SerializeMap` implementations are required to support
// `serialize_key` and `serialize_value` individually.
//
// There is a third optional method on the `SerializeMap` trait. The
// `serialize_entry` method allows serializers to optimize for the case where
// key and value are both available simultaneously. In JSON it doesn't make a
// difference so the default behavior for `serialize_entry` is fine.
impl<'a> ser::SerializeMap for &'a mut Serializer<'a> {
    type Ok = ();
    type Error = Error;

    // The Serde data model allows map keys to be any serializable type. JSON
    // only allows string keys so the implementation below will produce invalid
    // JSON if the key serializes as something other than a string.
    //
    // A real JSON serializer would need to validate that map keys are strings.
    // This can be done by using a different Serializer to serialize the key
    // (instead of `self`) and having that other serializer only
    // implement `serialize_str` and return an error on any other data type.
    fn serialize_key<T>(&mut self, key: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        key.serialize(*self)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(*self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.formatter.end_compound()?;
        Ok(())
    }
}

// TODO
impl<'a> ser::SerializeStruct for &'a mut Serializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        key.serialize(*self)?;
        value.serialize(*self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.formatter.end_compound()?;
        Ok(())
    }
}

// TODO
// Similar to `SerializeTupleVariant`, here the `end` method is responsible for
// closing both of the curly braces opened by `serialize_struct_variant`.
impl<'a> ser::SerializeStructVariant for &'a mut Serializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        key.serialize(*self)?;
        value.serialize(*self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.formatter.end_compound()?;
        self.formatter.end_compound()?;
        Ok(())
    }
}

// === === === === === === === === === === === === === === === === === ===
// === === === === === === === === === === === === === === === === === ===
// === === === === === === === === === === === === === === === === === ===

impl<'a> ser::SerializeMap for CompoundProxy<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        match *self {
            CompoundProxy::Map(ref mut proxy) => {
                // value.serialize(proxy.ser)?;
                Ok(())
            },
            _ => Err(Error::Invalid),
        }
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        match *self {
            CompoundProxy::Map(ref mut proxy) => {
                // value.serialize(proxy.ser)?;
                Ok(())
            },
            _ => Err(Error::Invalid),
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self {
            CompoundProxy::Map(proxy) => {
                // let ser = proxy.ser;

                Ok(())
            },
            _ => Err(Error::Invalid),
        }
    }
}

// === === === === === === === === === === === === === === === === === ===
// === === === === === === === === === === === === === === === === === ===
// === === === === === === === === === === === === === === === === === ===
// === === === === === === === === === === === === === === === === === ===
// === === === === === === === === === === === === === === === === === ===
// === === === === === === === === === === === === === === === === === ===

/**
 *
 */
struct Formatter<'a> {
    env: Env<'a>,
    proxies: Vec<Proxy<'a>>,
}

impl<'a> Formatter<'a> {
    pub fn write_primitive<T: Encoder>(&'a mut self, native: T) -> Result<(), Error> {
        let term = self.native_to_term(native);
        let mut_proxy = self.proxies.last_mut();
        match mut_proxy {
            Some(Proxy::Compound(compound)) => compound.add(term),
            Some(Proxy::Primitive(_)) => Err(Error::Invalid),
            None => {
                self.proxies.push(Proxy::Primitive(term));
                Ok(())
            },
        }
    }

    pub fn start_compound(&'a mut self, compound: CompoundProxy<'a>) -> () {
        self.proxies.push(Proxy::Compound(compound))
    }

    pub fn end_compound(&'a mut self) -> Result<(), Error> {
        let proxy = self.proxies.pop().ok_or(Error::Invalid)?;
        let term = proxy.to_term(self.env)?;

        if self.len() == 0 {
            // put it back so that Serializer can
            self.proxies.push(proxy);
            Ok(())
        } else {
            match self.proxies.pop().ok_or(Error::Invalid)? {
                Proxy::Compound(compound) => compound.add(term),
                Proxy::Primitive(_) => Err(Error::Invalid),
            }
        }
    }

    fn len(&self) -> usize { self.proxies.len() }

    fn native_to_term<T: Encoder>(&self, native: T) -> Term<'a> {
        native.encode(self.env)
    }
}

pub enum Proxy<'a> {
    Primitive(Term<'a>),
    Compound(CompoundProxy<'a>),
}

impl<'a> Proxy<'a> {
    pub fn to_term(&self, env: Env<'a>) -> Result<Term<'a>, Error> {
        match self {
            Proxy::Primitive(term) => Ok(*term),
            Proxy::Compound(compound) => compound.to_term(env),
        }
    }
}

/**
 *
 */
#[enum_dispatch(Compound)]
pub enum CompoundProxy<'a> {
    Seq(SequenceProxy<'a>),
    Map(MapProxy<'a>),
    Tuple(TupleProxy<'a>),
}

// TODO: refactor this to support all 7 compound types (even with small stubs)
// TODO: have internal Serializer types impl'ed for `CompoundProxy<'a>` (not &'a mut)
impl<'a> CompoundProxy<'a> {
    pub fn new_map(len: Option<usize>) -> CompoundProxy<'a> {
        CompoundProxy::Map(MapProxy::new(len))
    }

    pub fn new_sequence(len: Option<usize>) -> CompoundProxy<'a> {
        CompoundProxy::Seq(SequenceProxy::new(len))
    }

    pub fn new_tuple(len: Option<usize>) -> CompoundProxy<'a> {
        CompoundProxy::Tuple(TupleProxy::new(len))
    }
}

/**
 *
 */
#[enum_dispatch]
pub trait Compound<'a> {
    /**
     *
     */

    fn add(&mut self, term: Term<'a>) -> Result<(), Error>;

    fn to_term(&self, env: Env<'a>) -> Result<Term<'a>, Error>;
}

/**
 *
 */
pub struct MapProxy<'a> {
    keys: Vec<Term<'a>>,
    values: Vec<Term<'a>>,
}
impl<'a> MapProxy<'a> {
    pub fn new(len: Option<usize>) -> Self {
        match len {
            None => MapProxy {
                keys: Vec::new(),
                values: Vec::new(),
            },
            Some(length) => MapProxy {
                keys: Vec::with_capacity(length),
                values: Vec::with_capacity(length),
            },
        }
    }

    fn should_add_key(&self) -> bool {
        self.keys.len() == self.values.len()
    }
    fn should_add_val(&self) -> bool {
        self.keys.len() == self.values.len() + 1
    }
}
impl<'a> Compound<'a> for MapProxy<'a> {
    fn add(&mut self, term: Term<'a>) -> Result<(), Error> {
        if self.should_add_key() {
            self.keys.push(term);
            Ok(())
        } else if self.should_add_val() {
            self.values.push(term);
            Ok(())
        } else {
            Err(Error::Invalid)
        }
    }

    fn to_term(&self, env: Env<'a>) -> Result<Term<'a>, Error> {
        match Term::map_from_arrays(env, &self.keys, &self.values) {
            Err(_reason) => Err(Error::Invalid),
            Ok(term) => Ok(term),
        }
    }
}

/**
 *
 */
pub struct SequenceProxy<'a>(Vec<Term<'a>>);
impl<'a> SequenceProxy<'a> {
    pub fn new(len: Option<usize>) -> Self {
        match len {
            None => SequenceProxy(Vec::new()),
            Some(length) => SequenceProxy(Vec::with_capacity(length)),
        }
    }
}
impl<'a> Compound<'a> for SequenceProxy<'a> {
    fn add(&mut self, term: Term<'a>) -> Result<(), Error> {
        self.0.push(term);
        Ok(())
    }

    fn to_term(&self, env: Env<'a>) -> Result<Term<'a>, Error> {
        Ok(self.0.encode(env))
    }
}

/**
 *
 */
pub struct TupleProxy<'a>(Vec<Term<'a>>);
impl<'a> TupleProxy<'a> {
    pub fn new(len: Option<usize>) -> Self {
        match len {
            None => TupleProxy(Vec::new()),
            Some(length) => TupleProxy(Vec::with_capacity(length)),
        }
    }
}
impl<'a> Compound<'a> for TupleProxy<'a> {
    fn add(&mut self, term: Term<'a>) -> Result<(), Error> {
        self.0.push(term);
        Ok(())
    }

    fn to_term(&self, env: Env<'a>) -> Result<Term<'a>, Error> {
        Ok(tuple::make_tuple(env, &self.0))
    }
}


#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        assert!(true);
    }
}
