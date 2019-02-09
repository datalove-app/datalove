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

use std::collections::HashMap;
use crate::{
    ledger::{Ledger, LedgerIdRc},
    transactions::{
        base::TransactionId,
        context::MultiLedgerContext,
        basic::BasicTransaction,
        start_htl::StartHTLTransaction,
        end_htl::EndHTLTransaction,
        MultiLedgerTransaction,
        TransactionsMap as ITransactionsMap,
    },
};

pub type TransactionOrder = Vec<TransactionId>;
pub type TransactionOrders = HashMap<LedgerIdRc, TransactionOrder>;

#[derive(Default)]
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
pub struct MultiLedgerStateTree {
    // a set of all affected ledger ids (for convenience)
    // ledger_ids: LedgerIds,
    // a set of all affected ledgers and their potential ledger_contexts
    multiledger_context: MultiLedgerContext,
    // collection of all transactions relevant to the new transaction
    transactions: TransactionsMap,
    // an ordering of transactions for each ledger
    transaction_orders: TransactionOrders,
}

// PUBLIC METHODS
impl From<MultiLedgerTransaction> for MultiLedgerStateTree {
    // initializes a history around a new transaction
    fn from(tx: MultiLedgerTransaction) -> Self {
        let mut tx_map = TransactionsMap::new();
        tx_map.insert(tx);

        MultiLedgerStateTree {
            multiledger_context: MultiLedgerContext::new(),
            transactions: tx_map,
            transaction_orders: HashMap::new(),
        }
    }
}

impl MultiLedgerStateTree {
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
            // call LedgerOperationHistory::new; if successful, adds it to MultiLedgerStateTree

        self
    }
}

// PRIVATE METHODS
impl MultiLedgerStateTree {
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

    // validate the current transaction against the entire MultiLedgerStateTree
    pub fn validate(&self) -> Result<&Self, ()> {
        Err(())
    }

    //
    pub fn mut_apply(&mut self) -> Result<&mut Self, ()> {
        Err(())
    }
}
