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
use std::rc::Rc;
use crate::ledger::*;
use crate::transactions::{*, base::{MultiLedgerHistory, *}};
use super::operation::*;

/**
 * Stores a hashmap of `OperationHistory`s and any side effects of applying a
 * transaction.
 */
pub struct MultiLedgerState {
    histories: OperationHistories,
    effects: TransactionEffects,
}

impl MultiLedgerState {
    pub fn new() -> Self {
        MultiLedgerState{
            histories: HashMap::new(),
            effects: HashMap::new(),
        }
    }

    pub fn add_ledger(&mut self, ledger: Ledger) -> &mut Self {
        self.histories.insert(ledger.id(), OperationHistory::new(ledger));
        self
    }
}

impl MultiLedgerHistory for MultiLedgerState {
    fn has_history(&self, ledger_id: &LedgerId) -> bool {
        self.histories.contains_key(ledger_id)
    }

    fn has_all_histories(&self, required_ids: &LedgerIds) -> bool {
        required_ids.iter().all(|id| self.get(id).is_some())
    }

    fn get(&self, ledger_id: &LedgerId) -> Option<&OperationHistory> {
        self.histories.get(ledger_id)
    }

    fn iter(&self) -> Iter<LedgerId, OperationHistory> {
        self.histories.iter()
    }

    fn effects(&self) -> &TransactionEffects { &self.effects }
    fn mut_effects(&mut self) -> &mut TransactionEffects { &mut self.effects }
}

pub type OperationHistories = HashMap<LedgerId, OperationHistory>;
pub type TransactionOrder = Vec<TransactionId>;
pub type TransactionOrders = HashMap<LedgerId, TransactionOrder>;

/**
 * Contains:
 * - a `MultiLedgerState`,
 * - a map of `Transaction`s
 */
pub struct TransactionHistory {
    // a set of all affected ledger ids (for convenience)
    // ledger_ids: LedgerIds,
    // a set of all affected ledgers and their potential histories
    multiledger_historys: MultiLedgerState,
    // a list of all transactions
    transactions: TransactionMap,
    // an ordering of transactions for each ledger
    transaction_orders: TransactionOrders,
}

// PUBLIC METHODS
impl TransactionHistory {
    // initializes a history around a new transaction
    pub fn from_transaction(tx: MultiLedgerTransaction) -> Result<Self, ()> {
        let mut tx_map = HashMap::new();
        tx_map.insert(tx.id(), tx);

        Ok(TransactionHistory {
            multiledger_historys: MultiLedgerState::new(),
            transactions: tx_map,
            transaction_orders: HashMap::new(),
        })
    }

    // creates a new LedgerOperationHistory, applying each transaction
    pub fn mut_apply_ledger(
        &mut self,
        ledger: Ledger,
        transactions: Vec<MultiLedgerTransaction>
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
            MultiLedgerTransaction::Basic(tx) =>
                self.validate_basic(),
            MultiLedgerTransaction::StartHTL(tx) =>
                self.validate_start_htl(),
            MultiLedgerTransaction::EndHTL(tx) =>
                self.validate_end_htl(),
        }
    }

    fn mut_apply_transaction(
        &mut self,
        transaction: &MultiLedgerTransaction,
    ) -> Result<&mut Self, ()> {
        match transaction {
            MultiLedgerTransaction::Basic(tx) =>
                self.mut_apply_basic(),
            MultiLedgerTransaction::StartHTL(tx) =>
                self.mut_apply_start_htl(),
            MultiLedgerTransaction::EndHTL(tx) =>
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
