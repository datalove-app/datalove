use std::{
    collections::HashSet,
    rc::Rc,
};
use quick_error::quick_error;
use serde_derive::{Serialize, Deserialize};
use crate::types::AgentAddressRc;
use super::{
    base::{
        Context,
        HashedTimeLockProof,
        LedgerIds,
        LedgerOperations,
        SequenceNumbers,
        Transaction,
        TransactionId,
    },
    start_htl::StartHTLTransaction,
};

/**
 *
 */
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EndHTLTransaction {
    // TODO: could this be the start_htl_txid?
    // TODO: could this be the start_htl_txid AND the hashlock?
    // TODO: it can be both, just prefixed or suffixed with another identifier
    id: TransactionId,
    sender: AgentAddressRc,
    seq_nos: SequenceNumbers,
    start_htl_id: TransactionId,
    proof: HashedTimeLockProof,
}

impl EndHTLTransaction {
    /**
     */
    pub fn start_htl_id(&self) -> TransactionId {
        Rc::clone(&self.start_htl_id)
    }

    pub fn mut_validate_and_apply<C: Context>(
        &self,
        start_htl: &StartHTLTransaction,
        context: C,
    ) -> Result<C, Error> {
        // ensure all seq_ledger_ids in start_htl are listed in &self
        // ensure this txn's seq_no is one greater than seq_no in ledger

        Err(Error::InvalidTransaction)
    }

    pub fn required_ledger_ids(
        &self,
        start_htl: &StartHTLTransaction,
    ) -> Option<LedgerIds> {
        let end_htl_ledger_ids = self.seq_ledger_ids();
        start_htl
            .required_ledger_ids()
            .filter(|start_ids| end_htl_ledger_ids.eq(start_ids))
            .map(|_| end_htl_ledger_ids)
    }
}

impl Transaction for EndHTLTransaction {
    type Error = Error;

    fn id(&self) -> TransactionId { Rc::clone(&self.id) }
    fn seq_nos(&self) -> &SequenceNumbers { &self.seq_nos }
    fn operations(&self) -> Option<&LedgerOperations> { None }

    fn mut_validate_and_apply<C: Context>(
        &self,
        context: C,
    ) -> Result<C, Error> {
        panic!("Use `mut_validate_and_apply(&self, start_htl: &StartHTLTransaction, context: C)");
    }

    fn operation_ledger_ids(&self) -> LedgerIds { HashSet::new() }
    fn required_ledger_ids(&self) -> Option<LedgerIds> {
        panic!("Use `required_ledger_ids(&self, start_htl: &StartHTLTransaction)");
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        InvalidTransaction {
            description("Invalid transaction")
        }
    }
}
