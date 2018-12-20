use std::collections::HashMap;
use std::error::Error;
use crate::ledger::{Ledger, LedgerIdRc};

pub type OperationEffectKey = (&'static str, String);
pub type OperationEffects = HashMap<OperationEffectKey, String>;

/**
 * Provides access to the ledger and any effects an operation may have.
 */
pub trait OperationContext {
    fn sender(&self) -> &str;
    fn ledger(&self) -> &Ledger;
    fn effects(&self) -> &OperationEffects;

    fn mut_ledger(&mut self) -> &mut Ledger;
    fn mut_effects(&mut self) -> &mut OperationEffects;
}

/**
 * Validation and application of changes to a ledger.
 */
pub trait Operation<'a, OpError: Error> {
    /**
     * `LedgerIdRc` of the ledger to which the operation should be applied.
     */
    fn ledger_id(&self) -> LedgerIdRc;

    /**
     * Determines if the operation can be applied to a given `LedgerState`.
     */
    fn validate(
        &self,
        context: &OperationContext,
    ) -> Result<&Self, OpError>;

    /**
     * Applies the operation's changes to the underlying `LedgerState`.
     */
    fn mut_apply(
        &'a self,
        mut_context: &'a mut OperationContext
    ) -> &'a mut OperationContext;
}
