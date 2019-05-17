use self::error::Error;
use super::atoms;
use rustler::{dynamic, types::tuple, Encoder, Env, Term, TermType};
use serde::ser::{self, Serialize};

pub mod error;

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
    env: Env<'a>,
}

impl<'a> From<Env<'a>> for Serializer<'a> {
    fn from(env: Env<'a>) -> Serializer<'a> {
        Serializer { env: env }
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
impl<'a> Serializer<'a> {}

impl<'a> ser::Serializer for &'a Serializer<'a> {
    type Ok = Term<'a>;
    type Error = Error;

    type SerializeSeq = SequenceSerializer<'a>;
    type SerializeTuple = SequenceSerializer<'a>;
    type SerializeTupleStruct = SequenceSerializer<'a>;
    type SerializeTupleVariant = SequenceSerializer<'a>;
    type SerializeMap = MapSerializer<'a>;
    type SerializeStruct = MapSerializer<'a>;
    type SerializeStructVariant = MapSerializer<'a>;

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(atoms::nil().encode(self.env))
    }

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(v.encode(self.env))
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
        Ok(v.encode(self.env))
    }
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(v.encode(self.env))
    }
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(v.encode(self.env))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(v.encode(self.env))
    }
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(v.encode(self.env))
    }
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(v.encode(self.env))
    }
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(v.encode(self.env))
    }
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(v.encode(self.env))
    }
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(v.encode(self.env))
    }
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(v.encode(self.env))
    }

    //TODO
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    // TODO
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(v.encode(self.env))
    }

    // TODO
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        unimplemented!("return Binary or OwnedBinary?");
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    // TODO: could be an atom, but should probably be a string
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        unimplemented!("return atom or bytes?");
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
        let mut tuple = SequenceSerializer::new(&self, Some(2), None);
        let variant_term = atoms::try_from_str(self.env, variant).map_err(|_| Error::Invalid)?;
        tuple.add(variant_term)?;
        tuple.add(value.serialize(self)?)?;
        tuple.to_tuple()
    }

    // Now we get to the serialization of compound types.
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SequenceSerializer::new(&self, len, None))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(SequenceSerializer::new(&self, Some(len), None))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_tuple(len)
    }

    // TODO
    // Serializes `E::T` of `enum E { T(u8, u8) }` into `{:T, {u8, u8}}`
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        let variant_term = atoms::try_from_str(self.env, variant).map_err(|_| Error::Invalid)?;
        Ok(SequenceSerializer::new(
            &self,
            Some(len),
            Some(variant_term),
        ))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(MapSerializer::new(self, len, None, None))
    }

    // attempts to create __struct__ field pointing to module atom
    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        let name_term = atoms::try_from_str(self.env, name).map_err(|_| Error::Invalid)?;
        Ok(MapSerializer::new(self, Some(len), Some(name_term), None))
    }

    // TODO
    // Serializes `E::S` of `enum E { S { r: u8, g: u8, b: u8 } }` into `%{:E, %S{...}}`
    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        let name_term = atoms::try_from_str(self.env, name).map_err(|_| Error::Invalid)?;
        let variant_term = atoms::try_from_str(self.env, variant).map_err(|_| Error::Invalid)?;
        Ok(MapSerializer::new(
            self,
            Some(len),
            Some(name_term),
            Some(variant_term),
        ))
    }
}

// === === === === === === === === === === === === === === === === === ===
// === === === === === === === === === === === === === === === === === ===
// === === === === === === === === === === === === === === === === === ===
// === === === === === === === === === === === === === === === === === ===
// === === === === === === === === === === === === === === === === === ===
// === === === === === === === === === === === === === === === === === ===

impl<'a> ser::SerializeSeq for SequenceSerializer<'a> {
    type Ok = Term<'a>;
    type Error = Error;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        let term = value.serialize(self.ser)?;
        self.add(term);
        Ok(())
    }

    fn end(self) -> Result<Term<'a>, Error> {
        self.to_list()
    }
}

impl<'a> ser::SerializeTuple for SequenceSerializer<'a> {
    type Ok = Term<'a>;
    type Error = Error;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        let term = value.serialize(self.ser)?;
        self.add(term);
        Ok(())
    }

    fn end(self) -> Result<Term<'a>, Error> {
        self.to_tuple()
    }
}

impl<'a> ser::SerializeTupleStruct for SequenceSerializer<'a> {
    type Ok = Term<'a>;
    type Error = Error;

    // Serialize a single element of the sequence.
    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        let term = value.serialize(self.ser)?;
        self.add(term);
        Ok(())
    }

    fn end(self) -> Result<Term<'a>, Error> {
        self.to_tuple()
    }
}

impl<'a> ser::SerializeTupleVariant for SequenceSerializer<'a> {
    type Ok = Term<'a>;
    type Error = Error;

    // Serialize a single element of the sequence.
    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        let term = value.serialize(self.ser)?;
        self.add(term);
        Ok(())
    }

    fn end(self) -> Result<Term<'a>, Error> {
        self.to_tuple_variant()
    }
}

impl<'a> ser::SerializeMap for MapSerializer<'a> {
    type Ok = Term<'a>;
    type Error = Error;

    fn serialize_key<T>(&mut self, _value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        panic!("Not Implemented")
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        panic!("Not Implemented")
    }

    fn serialize_entry<K: ?Sized, V: ?Sized>(&mut self, key: &K, value: &V) -> Result<(), Error>
    where
        K: Serialize,
        V: Serialize,
    {
        self.add_key(key.serialize(self.ser)?)?;
        self.add_val(value.serialize(self.ser)?)?;
        Ok(())
    }

    fn end(self) -> Result<Term<'a>, Error> {
        self.to_map()
    }
}

impl<'a> ser::SerializeStruct for MapSerializer<'a> {
    type Ok = Term<'a>;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        self.add_key(key.serialize(self.ser)?)?;
        self.add_val(value.serialize(self.ser)?)?;
        Ok(())
    }

    fn end(self) -> Result<Term<'a>, Error> {
        self.to_struct()
    }
}

impl<'a> ser::SerializeStructVariant for MapSerializer<'a> {
    type Ok = Term<'a>;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        self.add_key(key.serialize(self.ser)?)?;
        self.add_val(value.serialize(self.ser)?)?;
        Ok(())
    }

    fn end(self) -> Result<Term<'a>, Error> {
        self.to_struct_variant()
    }
}

/**
 *
 */
pub struct MapSerializer<'a> {
    ser: &'a Serializer<'a>,
    name: Option<Term<'a>>,
    variant: Option<Term<'a>>,
    keys: Vec<Term<'a>>,
    values: Vec<Term<'a>>,
}
impl<'a> MapSerializer<'a> {
    pub fn new(
        ser: &'a Serializer<'a>,
        len: Option<usize>,
        name: Option<Term<'a>>,
        variant: Option<Term<'a>>,
    ) -> Self {
        match len {
            None => MapSerializer {
                ser,
                name,
                variant,
                keys: Vec::new(),
                values: Vec::new(),
            },
            Some(length) => MapSerializer {
                ser,
                name,
                variant,
                keys: Vec::with_capacity(length),
                values: Vec::with_capacity(length),
            },
        }
    }

    pub fn add_key(&mut self, term: Term<'a>) -> Result<(), Error> {
        Ok(self.keys.push(term))
    }

    pub fn add_val(&mut self, term: Term<'a>) -> Result<(), Error> {
        Ok(self.values.push(term))
    }

    pub fn to_map(&self) -> Result<Term<'a>, Error> {
        match Term::map_from_arrays(self.ser.env, &self.keys, &self.values) {
            Err(_reason) => Err(Error::Invalid),
            Ok(term) => Ok(term),
        }
    }

    // TODO: support :__struct__ and atom key? is name/variant correct?
    pub fn to_struct(&self) -> Result<Term<'a>, Error> {
        let module_term = self.name.ok_or(Error::Invalid)?;
        let struct_atom = atoms::__struct__().to_term(self.ser.env);
        self.to_map()?
            .map_put(struct_atom, module_term)
            .map_err(|_| Error::Invalid)
    }

    // TODO: support :__struct__ and atom key? is name/variant correct?
    pub fn to_struct_variant(&self) -> Result<Term<'a>, Error> {
        let variant_term = self.variant.ok_or(Error::Invalid)?;
        let struct_term = self.to_struct()?;
        Ok(tuple::make_tuple(
            self.ser.env,
            &vec![variant_term, struct_term],
        ))
    }
}

/**
 *
 */
pub struct SequenceSerializer<'a> {
    ser: &'a Serializer<'a>,
    variant: Option<Term<'a>>,
    items: Vec<Term<'a>>,
}
impl<'a> SequenceSerializer<'a> {
    pub fn new(ser: &'a Serializer<'a>, len: Option<usize>, variant: Option<Term<'a>>) -> Self {
        match len {
            None => SequenceSerializer {
                ser,
                variant,
                items: Vec::new(),
            },
            Some(length) => SequenceSerializer {
                ser,
                variant,
                items: Vec::with_capacity(length),
            },
        }
    }

    pub fn add(&mut self, term: Term<'a>) -> Result<(), Error> {
        self.items.push(term);
        Ok(())
    }

    pub fn to_list(&self) -> Result<Term<'a>, Error> {
        Ok(self.items.encode(self.ser.env))
    }

    pub fn to_tuple(&self) -> Result<Term<'a>, Error> {
        Ok(tuple::make_tuple(self.ser.env, &self.items))
    }

    pub fn to_tuple_variant(&self) -> Result<Term<'a>, Error> {
        let variant_term = self.variant.ok_or(Error::Invalid)?;
        let tuple_term = self.to_tuple()?;
        Ok(tuple::make_tuple(
            self.ser.env,
            &vec![variant_term, tuple_term],
        ))
    }
}

// === === === === === === === === === === === === === === === === === ===
// === === === === === === === === === === === === === === === === === ===
// === === === === === === === === === === === === === === === === === ===
// === === === === === === === === === === === === === === === === === ===
// === === === === === === === === === === === === === === === === === ===
// === === === === === === === === === === === === === === === === === ===

// TODO: refactor this to support all 7 compound types (even with small stubs)
// TODO: have internal Serializer types impl'ed for `CompoundProxy<'a>` (not &'a mut)

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        assert!(true);
    }
}
