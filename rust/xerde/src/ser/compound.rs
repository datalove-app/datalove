use super::error::Error;
use rustler::{types::tuple, Encoder, Env, Term};

#[enum_dispatch(Compound)]
pub enum CompoundProxy<'a> {
    Seq(SequenceProxy<'a>),
    Map(MapProxy<'a>),
    Tuple(TupleProxy<'a>),
}

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
