use std::rc::Rc;
use quick_error::quick_error;
use serde_derive::{Serialize, Deserialize};
use super::base::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct StartHTLTransaction {
    id: TransactionId, // TODO: could this be the hashlock itself?
    sender: TransactionAgent,
    seq_nos: SequenceNumbers,
    destination: TransactionAgent,

    /// a sender-specified seed to be concatenated with the preimage to
    /// generate the hashlock
    // hashlock_seed: Option<String>,
    // hashlock: String,

    // TODO: could this be used as a hashlock seed?
    metadata: Option<TransactionMetadata>,
    operations: Operations,
}

impl StartHTLTransaction {
    pub fn mut_validate_and_apply<S: MultiLedgerState>(
        &self,
        _multiledger_state: S,
    ) -> Result<S, Error> {
        // ensure no ops require ledgers not listed in seq_nos
        // ensure this txn's seq_no is one greater than seq_no in ledger
        // ensure that each operation is valid and applied

        Err(Error::InvalidTransaction)
    }
}

impl Transaction<Error> for StartHTLTransaction {
    fn id(&self) -> TransactionId { Rc::clone(&self.id) }
    fn seq_nos(&self) -> &SequenceNumbers { &self.seq_nos }
    fn operations(&self) -> Option<&Operations> { Some(&self.operations) }
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        InvalidTransaction {
            description("Invalid transaction")
        }
    }
}
