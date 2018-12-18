use std::collections::HashMap;
use crate::{
    ledger::Ledger,
    operations::{
        base::{
            OperationContext as IOperationContext,
            OperationEffects,
        },
        LedgerOperation,
        Error as LedgerOperationError,
    },
};

/**
 * Stores the ledger state and any side effects of applying an operation
 */
pub struct OperationContext {
    sender: &'static str,
    ledger: Ledger,
    effects: OperationEffects,
}

impl OperationContext {
    pub fn new(ledger: Ledger) -> Self {
        OperationContext {
            sender: "",
            ledger,
            effects: HashMap::new()
        }
    }
}

impl IOperationContext for OperationContext {
    fn sender(&self) -> &str { &self.sender }
    fn ledger(&self) -> &Ledger { &self.ledger }
    fn effects(&self) -> &OperationEffects { &self.effects }

    fn mut_ledger(&mut self) -> &mut Ledger { &mut self.ledger }
    fn mut_effects(&mut self) -> &mut OperationEffects { &mut self.effects }
}

/**
 * Contains a list of `OperationContext`s, i.e. the entire potential history
 * of a single ledger.
 *
 * Since operations within HTL transactions can succeed or fail
 * `OperationContext`s are either cloned or removed from the
 * `SingleLedgerContexts` before the newer operations are applied. This allows
 * certain operations to be applied on top of currently unresolved operations.
 */
pub struct SingleLedgerContexts(Vec<OperationContext>);

impl SingleLedgerContexts {
    pub fn from(ledger: Ledger) -> Self {
        let mut contexts = Vec::new();
        contexts.push(OperationContext::new(ledger));
        SingleLedgerContexts(contexts)
    }

    pub fn current_seq_no(&self) -> Option<u64> {
        self.0
            .first()
            .map(|context| context.ledger().seq_no())
    }

    pub fn validate(
        &self,
        operation: &LedgerOperation,
    ) -> Result<&Self, LedgerOperationError> {
        self.0
            .iter()
            .fold(Ok(()), |contexts_are_valid, context| {
                contexts_are_valid
                    .and_then(|_| operation.validate(context))
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
