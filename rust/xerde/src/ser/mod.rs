use quick_error::quick_error;
use rustler::{
    dynamic,
    Env,
    Encoder,
    Term,
    TermType,
};
use serde::ser;

/**
 *
 */
pub struct SerializerOutput<'a>(Option<Term<'a>>);

impl<'a> SerializerOutput<'a> {
    fn init(term: Term) -> Self {
        Self(Some(term))
    }
}

/**
 * maintains the stack of nested terms, the last being the current associated term
 * when serializing a value, the context can determine how to add the element to the given context
 */
pub struct SerializerContext<'a>(Vec<&'a mut Term<'a>>);

/**
 * initial primitive
 * - set to output
 *
 * initial associated
 * - set to output
 * - context.new
 *
 * additional primitive (within an associated)
 * - get context
 * - context.append (or append_key, append_value)
 *  -- delegates to TermType specific method for adding to the context
 *  -- mutation creates a new context (b/c of immutability)
 * - replace last context with new
 *
 * associated start
 * -
 *
 * associated end
 * - context.pop
 * - replace itself
 */
impl<'a> SerializerContext<'a> {
    fn mut_current(&'a self) -> Option<&'a mut Term<'a>> {
        match self.0.last_mut() {
            None => None,
            Some(&mut term) => Some(term),
        }
    }

    // fn term_type(&self) -> TermType {

    // }
}

/**
 *
 * maintains the accumulation of serializing Serde's data model into Elixir terms
 *
 * output: is either set to a primitive, or to an associated type
 *  - if primitive, then good to go
 *  - if associated, then
 */
pub struct Serializer<'a> {
    env: Env<'a>,
    context_stack: Vec<&'a mut Term<'a>>,
    output: Option<Term<'a>>,
}

impl<'a> Serializer<'a> {
    fn add_term<T: Encoder>(&'a mut self, native: T) -> () {
        if self.is_first() {
            return self.add_initial_term(native);
        }

        let context = self.mut_context();
        let term = native.encode(self.env());
        if is_primitive_term(&term) {

        }

        ()
    }

    fn add_primitive_term<T: Encoder>(&mut self, native: T) -> () {
        ()
    }

    fn add_associated_term<T: Encoder>(&mut self, native: T) -> () {
        ()
    }

    fn env(&self) -> Env<'a> { self.env }

    fn is_first(&self) -> bool { self.output.is_none() }

    fn add_initial_term<T: Encoder>(&mut self, native: T) -> () {
        let term = native.encode(self.env());
        self.output = Some(term);
        if is_associated_term(&term) {
            self.context_stack.push(&mut self.output.unwrap());
        }
    }


}

impl<'a> ser::Serializer for &mut Serializer<'a> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.add_primitive_term(v);
        Ok(())
    }
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
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Invalid {
            description("invalid")
        }
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
