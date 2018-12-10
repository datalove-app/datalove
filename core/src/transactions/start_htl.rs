use std::rc::Rc;
use quick_error::quick_error;
use serde_derive::{Serialize, Deserialize};
use crate::types::*;
use super::base::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct StartHTLTransaction<'a> {
    #[serde(borrow)]
    id: TransactionId<'a>, // TODO: could this be the hashlock itself?

    sender: Rc<Hash>,

    #[serde(borrow)]
    seq_nos: SequenceNumbers<'a>,

    destination: Hash,

    /// a sender-specified seed to be concatenated with the preimage to
    /// generate the hashlock
    // hashlock_seed: Option<Hash>,
    // hashlock: Hash,

    // TODO: could this be used as a hashlock seed?
    metadata: Option<TransactionMetadata>,

    #[serde(borrow)]
    operations: Operations<'a>,
}

impl<'a> StartHTLTransaction<'a> {
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

impl<'a> Transaction<'a, Error> for StartHTLTransaction<'a> {
    fn id(&self) -> TransactionId<'a> { &self.id }
    fn seq_nos(&self) -> &SequenceNumbers<'a> { &self.seq_nos }
    fn operations(&self) -> Option<&Operations<'a>> { Some(&self.operations) }
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        InvalidTransaction {
            description("Invalid transaction")
        }
    }
}
