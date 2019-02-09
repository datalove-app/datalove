use std::rc::Rc;
use quick_error::quick_error;
use serde_derive::{Serialize, Deserialize};
use crate::types::AgentAddressRc;
use super::base::{
    Context,
    LedgerOperations,
    SequenceNumbers,
    Transaction,
    TransactionId,
    Metadata,
};

/**
 *
 */
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BasicTransaction {
    id: TransactionId,
    sender: AgentAddressRc,
    seq_nos: SequenceNumbers,
    metadata: Option<Metadata>,
    operations: LedgerOperations,
}

impl Transaction for BasicTransaction {
    type Error = Error;

    fn id(&self) -> TransactionId { Rc::clone(&self.id) }
    fn seq_nos(&self) -> &SequenceNumbers { &self.seq_nos }
    fn operations(&self) -> Option<&LedgerOperations> { Some(&self.operations) }

    fn mut_validate_and_apply<C: Context>(
        &self,
        context: C,
    ) -> Result<C, Error> {
        // ensure no ops require ledgers not listed in seq_nos
        // ensure sender is owner on all used ledgers
        // ensure this txn's seq_no is one greater than seq_no in ledger
        // ensure that each operation is valid and applied

        Err(Error::InvalidTransaction)
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
