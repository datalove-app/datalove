use std::collections::HashMap;
use crate::ledger::{Ledger, LedgerIdRc};

pub type EffectKey = (&'static str, String);
pub type Effects = HashMap<EffectKey, String>;

/**
 * Provides access to the ledger and any effects an operation may have.
 */
pub trait Context {
    fn sender(&self) -> &str;
    fn ledger(&self) -> &Ledger;
    fn effects(&self) -> &Effects;

    fn mut_ledger(&mut self) -> &mut Ledger;
    fn mut_effects(&mut self) -> &mut Effects;
}

/**
 * Validation and application of changes to a ledger.
 */
pub trait Operation<'a> {
    type Error;

    /**
     * `LedgerIdRc` of the ledger to which the operation should be applied.
     */
    fn ledger_id(&self) -> LedgerIdRc;

    /**
     * Determines if the operation can be applied to a given `LedgerState`.
     */
    fn validate(
        &self,
        context: &Context,
    ) -> Result<&Self, Self::Error>;

    /**
     * Applies the operation's changes to the underlying `LedgerState`.
     */
    fn mut_apply(
        &'a self,
        mut_context: &'a mut Context
    ) -> &'a mut Context;
}
