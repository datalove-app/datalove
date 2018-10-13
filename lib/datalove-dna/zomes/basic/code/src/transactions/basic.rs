use std::rc::Rc;
use types::*;
use super::base::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct BasicTransaction {
	id: Rc<Hash>,
	sender: Rc<Hash>,
	seq_nos: SequenceNumbers,
	metadata: Option<TransactionMetadata>,
	operations: Operations,
}

impl BasicTransaction {
	pub fn validate_and_apply(
		&self,
		ledger_ids: &LedgerIds,
		multiledger_state: MultiLedgerState,
	) -> Result<MultiLedgerState, BasicTransactionError> {
		// ensure no ops require ledgers not listed in seq_nos
		// ensure sender is owner on all used ledgers
		// ensure this txn's seq_no is one greater than seq_no in ledger

		Err(BasicTransactionError::InvalidTransaction)
	}
}

impl<'a> Transaction<'a, BasicTransactionError> for BasicTransaction {
	fn id(&self) -> Rc<Hash> { Rc::clone(&self.id) }
	fn operations(&self) -> &Operations { &self.operations }
	fn seq_nos(&self) -> &SequenceNumbers { &self.seq_nos }

	// fn validate(
	// 	&self
	// ) -> Result<&Self, BasicTransactionError> {
	// 	Ok(self)
	// }

	// fn mut_apply(&self, ledgers: &mut Vec<Ledger>) {}
}

quick_error! {
	#[derive(Debug)]
	pub enum BasicTransactionError {
		InvalidTransaction {
			description("Invalid transaction")
		}
	}
}
