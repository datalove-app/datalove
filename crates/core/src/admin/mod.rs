use crate::dev::*;

pub trait Admin<C: Core>: Service<C> {}
