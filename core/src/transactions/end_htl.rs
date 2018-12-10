use std::{
    collections::HashSet,
    rc::Rc,
};
use quick_error::quick_error;
use serde_derive::{Serialize, Deserialize};
use crate::types::*;
use super::{
    base::*,
    start_htl::StartHTLTransaction,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct EndHTLTransaction<'a> {
    // TODO: could this be the start_htl_txid?
    // TODO: could this be the start_htl_txid AND the hashlock?
    #[serde(borrow)]
    id: TransactionId<'a>,

    sender: Rc<Hash>,

    #[serde(borrow)]
    seq_nos: SequenceNumbers<'a>,

    #[serde(borrow)]
    start_htl_id: TransactionId<'a>,

    proof: HashedTimeLockProof,
}

impl<'a> EndHTLTransaction<'a> {
    pub fn start_htl_id(&self) -> TransactionId<'a> { &self.start_htl_id }

    pub fn validate_and_apply<H: MultiLedgerHistory>(
        &self,
        _start_htl: &StartHTLTransaction,
        _multiledger_history: H,
    ) -> Result<H, Error> {
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

impl<'a> Transaction<'a, Error> for EndHTLTransaction<'a> {
    fn id(&self) -> TransactionId<'a> { &self.id }
    fn seq_nos(&self) -> &SequenceNumbers<'a> { &self.seq_nos }
    fn operations(&self) -> Option<&Operations<'a>> { None }

    fn operation_ledger_ids(&self) -> LedgerIds<'a> { HashSet::new() }
    fn required_ledger_ids(&self) -> Option<LedgerIds<'a>> {
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
