/*  ## Tree based algo:
    on basic:
        map ledger tree
            bump seq_no
            apply tx
    on htl:
        map ledger tree
            bump seq_no
        traverse ledger tree
            apply tx
            append resulting ledger to each existing ledger as a child
    on htl_end:
        map ledger tree
            bump seq_no
        if htl failure tx:
            remove all ledgers (and subtrees) w/ the htl_id as key
        if htl fulfilled tx:
            traverse tree (in-order?)
            if ledger has htl_id as key
                delete newer, "younger" siblings (and subtrees)

 */

use std::collections::{hash_map::Iter, HashMap};
use crate::{
    ledger::{Ledger, LedgerIdRc},
    transactions::{
        base::{
            LedgerIds,
            TransactionContext as ITransactionContext,
            TransactionEffects,
            TransactionId,
        },
        basic::BasicTransaction,
        start_htl::StartHTLTransaction,
        end_htl::EndHTLTransaction,
        MultiLedgerTransaction,
        Error as MultiLedgerTransactionError,
        TransactionsMap as ITransactionsMap,
    },
};
use super::ledger::*;

pub type LedgerContexts = HashMap<LedgerIdRc, SingleLedgerContexts>;
pub type TransactionOrder = Vec<TransactionId>;
pub type TransactionOrders = HashMap<LedgerIdRc, TransactionOrder>;

/**
 * Stores a hashmap of `SingleLedgerContexts`s and any side effects of applying
 * a transaction.
 */
pub struct MultiLedgerContext {
    ledger_contexts: LedgerContexts,
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
            .insert(ledger.id(), SingleLedgerContexts::from(ledger));
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
    ) -> Option<&SingleLedgerContexts> {
        self.ledger_contexts.get(ledger_id)
    }

    fn ledger_iter(&self) -> Iter<LedgerIdRc, SingleLedgerContexts> {
        self.ledger_contexts.iter()
    }

    fn effects(&self) -> &TransactionEffects { &self.effects }
    fn mut_effects(&mut self) -> &mut TransactionEffects { &mut self.effects }
}

pub struct TransactionsMap(HashMap<TransactionId, MultiLedgerTransaction>);

impl TransactionsMap {
    pub fn new() -> Self { TransactionsMap(HashMap::new()) }

    pub fn insert(
        &mut self,
        tx: MultiLedgerTransaction
    ) -> Option<MultiLedgerTransaction> {
        self.0.insert(tx.id(), tx)
    }
}

impl ITransactionsMap for TransactionsMap {
    fn get(&self, id: &TransactionId) -> Option<&MultiLedgerTransaction> {
        self.0.get(id)
    }

    fn get_basic(
        &self,
        id: &TransactionId
    ) -> Option<&BasicTransaction> {
        self.0
            .get(id)
            .and_then(|tx| match tx {
                MultiLedgerTransaction::Basic(tx) => Some(tx),
                _ => None,
            })
    }

    fn get_start_htl(
        &self,
        id: &TransactionId
    ) -> Option<&StartHTLTransaction> {
        self.0
            .get(id)
            .and_then(|tx| match tx {
                MultiLedgerTransaction::StartHTL(tx) => Some(tx),
                _ => None,
            })
    }

    fn get_end_htl(
        &self,
        id: &TransactionId
    ) -> Option<&EndHTLTransaction> {
        self.0
            .get(id)
            .and_then(|tx| match tx {
                MultiLedgerTransaction::EndHTL(tx) => Some(tx),
                _ => None,
            })
    }
}

/**
 * TODO: rename, possibly move this somewhere else (in DNA code perhaps?)
 * Contains:
 * - a `MultiLedgerContext`,
 * - a map of `Transaction`s
 */
pub struct TransactionHistory {
    // a set of all affected ledger ids (for convenience)
    // ledger_ids: LedgerIds,
    // a set of all affected ledgers and their potential ledger_contexts
    multiledger_histories: MultiLedgerContext,
    // a list of all transactions
    transactions: TransactionsMap,
    // an ordering of transactions for each ledger
    transaction_orders: TransactionOrders,
}

// PUBLIC METHODS
impl TransactionHistory {
    // initializes a history around a new transaction
    pub fn from_transaction(tx: MultiLedgerTransaction) -> Result<Self, ()> {
        let mut tx_map = TransactionsMap::new();
        tx_map.insert(tx);

        Ok(TransactionHistory {
            multiledger_histories: MultiLedgerContext::new(),
            transactions: tx_map,
            transaction_orders: HashMap::new(),
        })
    }

    // creates a new LedgerOperationHistory, applying each transaction
    pub fn mut_apply_ledger(
        &mut self,
        _ledger: Ledger,
        _transactions: Vec<MultiLedgerTransaction>
    ) -> &Self {
        // validates new transaction against to-be-added ledger
            // i.e. if basic, are we the owner? etc
        // validates new transaction against transaction history
            // are there gaps in seq_no, and if not, does it end in one less than current transactions seq_no?
        // if valid
            // call LedgerOperationHistory::new; if successful, adds it to TransactionHistory

        self
    }
}

// PRIVATE METHODS
impl TransactionHistory {
    fn validate_transaction(
        &self,
        transaction: &MultiLedgerTransaction
    ) -> Result<&Self, ()> {
        match transaction {
            MultiLedgerTransaction::Basic(_tx) =>
                self.validate_basic(),
            MultiLedgerTransaction::StartHTL(_tx) =>
                self.validate_start_htl(),
            MultiLedgerTransaction::EndHTL(_tx) =>
                self.validate_end_htl(),
        }
    }

    fn mut_apply_transaction(
        &mut self,
        transaction: &MultiLedgerTransaction,
    ) -> Result<&mut Self, ()> {
        match transaction {
            MultiLedgerTransaction::Basic(_tx) =>
                self.mut_apply_basic(),
            MultiLedgerTransaction::StartHTL(_tx) =>
                self.mut_apply_start_htl(),
            MultiLedgerTransaction::EndHTL(_tx) =>
                self.mut_apply_end_htl(),
        }
    }

    fn validate_basic(
        &self,
        // transaction: &basic::BasicTransaction,
    ) -> Result<&Self, ()> {
        // self.validate_ops(transaction.operations())?;

        Ok(self)
    }

    fn validate_start_htl(
        &self,
        // transaction: &start_htl::StartHTLTransaction
    ) -> Result<&Self, ()> {
        Ok(self)
    }

    fn validate_end_htl(
        &self,
        // transaction: &end_htl::EndHTLTransaction
    ) -> Result<&Self, ()> {
        Ok(self)
    }

    fn mut_apply_basic(
        &mut self,
        // transaction: &basic::BasicTransaction,
    ) -> Result<&mut Self, ()> {
        // for each existing ledger in self
            // validate transaction against ledger
            // for each operation for this ledger
                // validate the operation against ledger (and history?)
            // if both transaction and all operations are valid
                // apply operations to ledger

        Ok(self)
    }

    fn mut_apply_start_htl(
        &mut self,
        // transaction: &start_htl::StartHTLTransaction
    ) -> Result<&mut Self, ()> {
        // for each existing ledger in self
            // validate transaction against ledger
            // for each operation for this ledger
                // validate the operation against ledger (and history?)
            // if both transaction and all operations are valid
                // clone this ledger
                // apply operations to ledger clone

        Ok(self)
    }

    fn mut_apply_end_htl(
        &mut self,
        // transaction: &end_htl::EndHTLTransaction
    ) -> Result<&mut Self, ()> {
        // for each existing ledger in self
            // validate transaction against ledger
                // i.e. ensure that sequence numbers match up
            // transaction is valid
                // delete ledger clones that don't include the successful start_htl

        Ok(self)
    }

    ////////////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////////

    // validate the current transaction against the entire TransactionHistory
    pub fn validate(&self) -> Result<&Self, ()> {
        Err(())
    }

    //
    pub fn mut_apply(&mut self) -> Result<&mut Self, ()> {
        Err(())
    }
}
