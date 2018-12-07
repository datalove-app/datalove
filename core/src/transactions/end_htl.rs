use std::{
    collections::HashSet,
    rc::Rc,
};
use crate::{
    ledger::LedgerId,
    types::*,
};
use super::{
    base::*,
    start_htl::StartHTLTransaction,
};

lazy_static! {
    static ref EMPTY_OP_LIST: Operations = Vec::new();
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndHTLTransaction {
    // TODO: could this be the start_htl_txid?
    // TODO: could this be the start_htl_txid AND the hashlock?
    id: TransactionId,
    sender: Rc<Hash>,
    seq_nos: SequenceNumbers,
    start_tx_hash: Rc<Hash>,
    proof: HashedTimeLockProof,
}

impl EndHTLTransaction {
    pub fn start_htl_hash(&self) -> Rc<Hash> { Rc::clone(&self.start_tx_hash) }

    pub fn validate_and_apply<H: MultiLedgerHistory>(
        &self,
        start_htl: &StartHTLTransaction,
        multiledger_history: H,
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

impl Transaction<Error> for EndHTLTransaction {
    fn id(&self) -> TransactionId { Rc::clone(&self.id) }
    fn operations(&self) -> &Operations { &EMPTY_OP_LIST }
    fn seq_nos(&self) -> &SequenceNumbers { &self.seq_nos }

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
