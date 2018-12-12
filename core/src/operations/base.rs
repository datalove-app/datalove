use std::collections::HashMap;
use std::error::Error;
use crate::ledger::*;

pub type LedgerEffectKey = (&'static str, String);
pub type LedgerEffects = HashMap<LedgerEffectKey, String>;

/**
 * Provides access to the ledger and any effects an operation may have.
 */
pub trait LedgerHistory {
    fn ledger(&self) -> &Ledger;
    fn effects(&self) -> &LedgerEffects;

    fn mut_ledger(&mut self) -> &mut Ledger;
    fn mut_effects(&mut self) -> &mut LedgerEffects;
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
     * Determines if the operation can be applied to a given `LedgerHistory`.
     */
    fn validate(
        &self,
        // tx_sender: &String,
        ledger_history: &LedgerHistory,
    ) -> Result<&Self, OpError>;

    /**
     * Applies the operation's changes to the underlying `LedgerHistory`.
     */
    fn mut_apply(
        &'a self,
        // tx_sender: &String,
        mut_ledger_history: &'a mut LedgerHistory,
    ) -> &'a mut LedgerHistory;
}
