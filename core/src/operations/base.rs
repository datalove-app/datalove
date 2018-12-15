use std::collections::HashMap;
use std::error::Error;
use crate::ledger::*;

pub type OperationEffectKey = (&'static str, String);
pub type OperationEffects = HashMap<OperationEffectKey, String>;

/**
 * Provides access to the ledger and any effects an operation may have.
 */
pub trait LedgerState {
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
     * `LedgerId` of the ledger to which the operation should be applied.
     */
    fn ledger_id(&self) -> LedgerId;

    /**
     * Determines if the operation can be applied to a given `LedgerState`.
     */
    fn validate(
        &self,
        // tx_sender: &String,
        ledger_state: &LedgerState,
    ) -> Result<&Self, OpError>;

    /**
     * Applies the operation's changes to the underlying `LedgerState`.
     */
    fn mut_apply(
        &'a self,
        // tx_sender: &String,
        mut_ledger_state: &'a mut LedgerState,
    ) -> &'a mut LedgerState;
}
