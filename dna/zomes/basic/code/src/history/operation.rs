use std::collections::HashMap;
use ledger::*;
use operations::{LedgerOperation, Error as LedgerOperationError};
use operations::base::{LedgerEffects, LedgerHistory, Operation};

/**
 * Stores the ledger state and any side effects of applying an operation
 */
pub struct SingleLedgerState {
    ledger: Ledger,
    effects: LedgerEffects,
}

impl SingleLedgerState {
    pub fn new(ledger: Ledger) -> Self {
        SingleLedgerState {
            ledger,
            effects: HashMap::new()
        }
    }
}

impl LedgerHistory for SingleLedgerState {
    fn ledger(&self) -> &Ledger { &self.ledger }
    fn effects(&self) -> &LedgerEffects { &self.effects }

    fn mut_ledger(&mut self) -> &mut Ledger { &mut self.ledger }
    fn mut_effects(&mut self) -> &mut LedgerEffects { &mut self.effects }
}

pub type SingleLedgerStates = Vec<SingleLedgerState>;

/**
 * Contains a list of `SingleLedgerState`s, i.e. the entire potential history
 * of a single ledger.
 *
 * Since operations within HTL transactions can succeed or fail
 * `SingleLedgerState`s are either cloned or removed from the
 * `OperationHistory` before the newer operations are applied. This allows
 * certain operations to be applied on top of currently unresolved operations.
 */
pub struct OperationHistory {
    ledger_states: SingleLedgerStates,
}

impl OperationHistory {
    pub fn new(ledger: Ledger) -> Self {
        let mut ledger_states = Vec::new();
        ledger_states.push(SingleLedgerState::new(ledger));
        OperationHistory { ledger_states }
    }

    pub fn current_seq_no(&self) -> Option<u64> {
        self.ledger_states
            .first()
            .map(|ledger_states| ledger_states.ledger().seq_no())
    }

    pub fn validate(
        &self,
        operation: &LedgerOperation,
    ) -> Result<&Self, LedgerOperationError> {
        self.ledger_states
            .iter()
            .fold(Ok(()), |ledger_states_are_valid, ledger_states| {
                ledger_states_are_valid
                    .and(operation.validate(ledger_states))
                    .and(Ok(()))
            })
            .map(|_| self)
    }

    pub fn mut_apply(
        &mut self,
        operation: &LedgerOperation,
    ) -> &mut Self {
        self.ledger_states
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
