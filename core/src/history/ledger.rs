use std::collections::HashMap;
use crate::{
    ledger::*,
    operations::{
        LedgerOperation,
        Error as LedgerOperationError,
        base::{
            OperationEffects,
            LedgerState as ILedgerState,
            Operation,
        },
    },
};

/**
 * Stores the ledger state and any side effects of applying an operation
 */
pub struct LedgerState {
    ledger: Ledger,
    effects: OperationEffects,
}

impl LedgerState {
    pub fn new(ledger: Ledger) -> Self {
        LedgerState {
            ledger,
            effects: HashMap::new()
        }
    }
}

impl ILedgerState for LedgerState {
    fn ledger(&self) -> &Ledger { &self.ledger }
    fn effects(&self) -> &OperationEffects { &self.effects }

    fn mut_ledger(&mut self) -> &mut Ledger { &mut self.ledger }
    fn mut_effects(&mut self) -> &mut OperationEffects { &mut self.effects }
}

/**
 * Contains a list of `LedgerState`s, i.e. the entire potential history
 * of a single ledger.
 *
 * Since operations within HTL transactions can succeed or fail
 * `LedgerState`s are either cloned or removed from the
 * `SingleLedgerStates` before the newer operations are applied. This allows
 * certain operations to be applied on top of currently unresolved operations.
 */
pub struct SingleLedgerStates(Vec<LedgerState>);

impl SingleLedgerStates {
    pub fn from(ledger: Ledger) -> Self {
        let mut ledger_states = Vec::new();
        ledger_states.push(LedgerState::new(ledger));
        SingleLedgerStates(ledger_states)
    }

    pub fn current_seq_no(&self) -> Option<u64> {
        self.0
            .first()
            .map(|ledger_states| ledger_states.ledger().seq_no())
    }

    pub fn validate(
        &self,
        operation: &LedgerOperation,
    ) -> Result<&Self, LedgerOperationError> {
        self.0
            .iter()
            .fold(Ok(()), |ledger_states_are_valid, ledger_states| {
                ledger_states_are_valid
                    .and_then(|_| operation.validate(ledger_states))
                    .and(Ok(()))
            })
            .map(|_| self)
    }

    pub fn mut_apply(
        &mut self,
        operation: &LedgerOperation,
    ) -> &mut Self {
        self.0
            .iter_mut()
            .for_each(|mut_ls| { operation.mut_apply(mut_ls); });

        self
    }

    // pub fn mut_fork(&mut self) -> &mut Self {
    // 	Ok(self)
    // }

    // pub fn mut_join(&mut self, index: usize) -> Result<&mut Self, ()> {
    // 	Ok(self)
    // }
}
