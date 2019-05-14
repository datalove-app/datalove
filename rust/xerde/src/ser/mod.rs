use rustler::{dynamic, Encoder, Env, Term, TermType};
use ser::Serialize;
use serde::ser;
use super::atoms;
use self::{
    compound::{Compound, CompoundProxy},
    error::Error,
};

pub mod compound;
pub mod error;

/**
 *
 */
pub struct Serializer<'a> {
    env: Env<'a>,
    proxies: Vec<Proxy<'a>>,
    output: Option<Term<'a>>,
}

impl<'a> From<Env<'a>> for Serializer<'a> {
    fn from(env: Env<'a>) -> Serializer<'a> {
        Serializer {
            env: env,
            proxies: Vec::new(),
            output: None,
        }
    }
}

/**
 *
 */
pub enum Proxy<'a> {
    Primitive(Term<'a>),
    Compound(CompoundProxy<'a>),
}

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

// pub fn to_term<'a, T>(env: Env<'a>, value: &T) -> Result<Term<'a>> where T: Serialize {}

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
    fn add_primitive<T: Encoder>(self, native: T) -> Result<(), Error> {
        let term = self._native_to_term(native);
        self.add_term(term)
    }

    fn add_term(mut self, term: Term<'a>) -> Result<(), Error> {
        if (&self)._is_first() {
            self._set_output(term);
            return Ok(());
        }

        self._add_to_proxy(term)
    }

    fn start_compound(self, compound: CompoundProxy<'a>) -> Result<(), Error> {
        self._start_proxy(compound);
        Ok(())
    }

    fn end_compound(mut self) -> Result<(), Error> {
        let is_last = self._is_last();
        let proxy = self._end_proxy().ok_or(Error::Invalid)?;
        let term = self._proxy_to_term(proxy)?;
        self._add_to_proxy(term);

        if is_last {
            return Ok(self._set_output(term));
        }

        Ok(())
    }

    // === === === === === === === === === === === === === === === === === ===

    fn _is_first(&self) -> bool {
        self.proxies.len() == 0
    }

    fn _is_last(&self) -> bool {
        self.proxies.len() == 1 && self.output.is_none()
    }

    fn _native_to_term<T: Encoder>(&self, native: T) -> Term<'a> {
        native.encode(self.env)
    }

    fn _proxy_to_term(&self, proxy: Proxy<'a>) -> Result<Term<'a>, Error> {
        match proxy {
            Proxy::Primitive(term) => Ok(term),
            Proxy::Compound(compound) => compound.to_term(self.env),
        }
    }

    fn _add_to_proxy(&'a mut self, term: Term<'a>) -> Result<(), Error> {
        let mut_proxy = self._mut_proxy().ok_or(Error::Invalid)?;
        match mut_proxy {
            Proxy::Compound(compound) => compound.add(term),
            _ => Err(Error::Invalid),
        }
    }

    fn _start_proxy(&'a mut self, compound: CompoundProxy<'a>) -> () {
        self.proxies.push(Proxy::Compound(compound))
    }

    fn _end_proxy(&'a mut self) -> Option<Proxy<'a>> {
        self.proxies.pop()
    }

    fn _mut_proxy(&'a mut self) -> Option<&'a mut Proxy<'a>> {
        self.proxies.last_mut()
    }

    fn _set_output(&'a mut self, term: Term<'a>) -> () {
        self.output = Some(term);
    }
}

impl<'a> ser::Serializer for &'a mut Serializer<'a> {
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
        let term = atoms::nil().to_term(self.env);
        self.add_term(term)
    }

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.add_primitive(v)
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
        self.add_primitive(v)
    }
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.add_primitive(v)
    }
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.add_primitive(v)
    }

    // Not particularly efficient but this is example code anyway. A more
    // performant approach would be to use the `itoa` crate.
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.add_primitive(v)
    }
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.add_primitive(v)
    }
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.add_primitive(v)
    }
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.add_primitive(v)
    }
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.add_primitive(v)
    }
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.add_primitive(v)
    }
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.add_primitive(v)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
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
        self.add_primitive(v);
        Ok(())
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

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
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
        let proxy = CompoundProxy::new_map(Some(1));
        self.start_compound(proxy);
        self.add_primitive(variant);
        value.serialize(self)
    }

    // Now we get to the serialization of compound types.
    //
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let proxy = CompoundProxy::new_sequence(len);
        self.start_compound(proxy);
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        let proxy = CompoundProxy::new_tuple(Some(len));
        self.start_compound(proxy);
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_tuple(len)
    }

    // Serializes `E::T` of `enum E { T(u8, u8) }` into `%{"T" => (u8, u8)}`
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.serialize_map(Some(1));
        variant.serialize(self);
        self.serialize_tuple(len)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let proxy = CompoundProxy::new_map(len);
        self.start_compound(proxy);
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

    // Serializes `E::S` of `enum E { S { r: u8, g: u8, b: u8 } }` into `%{"S" => %{...}}`
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.serialize_map(Some(1));
        variant.serialize(self);
        self.serialize_map(Some(len))
    }
}

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
        self.end_compound()
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
        self.end_compound()
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
        self.end_compound()
    }
}

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
        self.end_compound();
        self.end_compound()
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
    // (instead of `&mut **self`) and having that other serializer only
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
        self.end_compound()
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        key.serialize(*self);
        value.serialize(*self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.end_compound()
    }
}

// Similar to `SerializeTupleVariant`, here the `end` method is responsible for
// closing both of the curly braces opened by `serialize_struct_variant`.
impl<'a> ser::SerializeStructVariant for &'a mut Serializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        key.serialize(*self);
        value.serialize(*self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.end_compound();
        self.end_compound()
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
