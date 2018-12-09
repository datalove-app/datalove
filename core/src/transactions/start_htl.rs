use std::rc::Rc;
use quick_error::quick_error;
use serde_derive::{Serialize, Deserialize};
use crate::{
    ledger::LedgerId,
    types::*,
};
use super::base::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct StartHTLTransaction {
    id: TransactionId, // TODO: could this be the hashlock itself?
    sender: Rc<Hash>,
    seq_nos: SequenceNumbers,
    destination: Hash,

    /// a sender-specified seed to be concatenated with the preimage to
    /// generate the hashlock
    // hashlock_seed: Option<Hash>,
    // hashlock: Hash,

    // TODO: could this be used as a hashlock seed?
    metadata: Option<TransactionMetadata>,
    operations: Operations,
}

impl StartHTLTransaction {
    pub fn validate_and_apply<H: MultiLedgerHistory>(
        &self,
        _multiledger_history: H,
    ) -> Result<H, Error> {
        // ensure no ops require ledgers not listed in seq_nos
        // ensure this txn's seq_no is one greater than seq_no in ledger
        // ensure that each operation is valid and applied

        Err(Error::InvalidTransaction)
    }
}

impl Transaction<Error> for StartHTLTransaction {
    fn id(&self) -> TransactionId { Rc::clone(&self.id) }
    fn operations(&self) -> &Operations { &self.operations }
    fn seq_nos(&self) -> &SequenceNumbers { &self.seq_nos }
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        InvalidTransaction {
            description("Invalid transaction")
        }
    }
}
