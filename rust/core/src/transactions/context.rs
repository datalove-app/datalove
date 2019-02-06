use std::collections::{hash_map::Iter, HashMap};
use crate::{
    ledger::{Ledger, LedgerIdRc},
    transactions::base::{
        Context as ITransactionContext,
        Effects as TransactionEffects,
        LedgerIds,
    },
    state_tree::ledger::LedgerStateTree,
};

pub type LedgerStateTrees = HashMap<LedgerIdRc, LedgerStateTree>;

/**
 * Stores a hashmap of `LedgerStateTree`s and any side effects of applying
 * a transaction.
 */
pub struct MultiLedgerContext {
    ledger_contexts: LedgerStateTrees,
    effects: TransactionEffects,
}

impl MultiLedgerContext {
    pub fn new() -> Self {
        MultiLedgerContext {
            ledger_contexts: HashMap::new(),
            effects: HashMap::new(),
        }
    }

    pub fn add_ledger(&mut self, ledger: Ledger) -> &mut Self {
        self.ledger_contexts
            .insert(ledger.id(), LedgerStateTree::from(ledger));
        self
    }
}

impl ITransactionContext for MultiLedgerContext {
    fn has_ledger(&self, ledger_id: &LedgerIdRc) -> bool {
        self.ledger_contexts.contains_key(ledger_id)
    }

    fn has_all_ledgers(&self, required_ids: &LedgerIds) -> bool {
        required_ids.iter().all(|id| self.has_ledger(id))
    }

    fn ledger_context(
        &self,
        ledger_id: &LedgerIdRc
    ) -> Option<&LedgerStateTree> {
        self.ledger_contexts.get(ledger_id)
    }

    fn ledger_iter(&self) -> Iter<LedgerIdRc, LedgerStateTree> {
        self.ledger_contexts.iter()
    }

    fn effects(&self) -> &TransactionEffects { &self.effects }
    fn mut_effects(&mut self) -> &mut TransactionEffects { &mut self.effects }
}
