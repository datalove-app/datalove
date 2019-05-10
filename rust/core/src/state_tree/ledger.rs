use crate::{
    ledger::Ledger,
    operations::{
        base::Context as IOperationContext,
        context::OperationContext,
        LedgerOperation,
        Error as LedgerOperationError,
    },
};

/**
 * TODO: move this to multiledger.rs?
 * Contains a list of `OperationContext`s, i.e. the entire potential history
 * of a single ledger.
 *
 * Because operations within HTL transactions can succeed or fail,
 * `OperationContext`s are either cloned or removed from the
 * `LedgerStateTree` before the newer operations are applied. This allows
 * certain operations to be applied on top of currently unresolved operations.
 */
pub struct LedgerStateTree(Vec<OperationContext>);

impl From<Ledger> for LedgerStateTree {
    fn from(ledger: Ledger) -> Self {
        let mut contexts = Vec::new();
        contexts.push(OperationContext::new(ledger));
        LedgerStateTree(contexts)
    }
}

impl LedgerStateTree {
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
