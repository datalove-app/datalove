use std::rc::Rc;
use crate::ledger::LedgerId;
use crate::types::*;
use super::base::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct BasicTransaction {
    id: LedgerId,
    sender: Rc<Hash>,
    seq_nos: SequenceNumbers,
    metadata: Option<TransactionMetadata>,
    operations: Operations,
}

impl BasicTransaction {
    pub fn validate_and_apply<H: MultiLedgerHistory>(
        &self,
        multiledger_history: H,
    ) -> Result<H, Error> {
        // ensure no ops require ledgers not listed in seq_nos
        // ensure sender is owner on all used ledgers
        // ensure this txn's seq_no is one greater than seq_no in ledger
        // ensure that each operation is valid and applied

        Err(Error::InvalidTransaction)
    }
}

impl Transaction<Error> for BasicTransaction {
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
