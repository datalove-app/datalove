use std::rc::Rc;
use types::*;
use super::base::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct StartHTLTransaction {
	id: Rc<Hash>,
	sender: Rc<Hash>,
	seq_nos: SequenceNumbers,
	destination: Hash,
	hashlock: Hash,
	metadata: Option<TransactionMetadata>,
	operations: Operations,
}

impl StartHTLTransaction {
	pub fn validate_and_apply(
		&self,
		ledger_ids: &LedgerIds,
		multiledger_state: MultiLedgerState,
	) -> Result<MultiLedgerState, StartHTLTransactionError> {
		// ensure no ops require ledgers not listed in seq_nos
		// ensure this txn's seq_no is one greater than seq_no in ledger

		Err(StartHTLTransactionError::InvalidTransaction)
	}
}

impl<'a> Transaction<'a, StartHTLTransactionError> for StartHTLTransaction {
	fn id(&self) -> Rc<Hash> { Rc::clone(&self.id) }
	fn operations(&self) -> &Operations { &self.operations }
	fn seq_nos(&self) -> &SequenceNumbers { &self.seq_nos }

	// fn validate(
	// 	&self,
	// ) -> Result<&Self, StartHTLTransactionError> {
	// 	Ok(self)
	// }

	// fn mut_apply(&self, ledgers: &mut Vec<Ledger>) {}
}

quick_error! {
	#[derive(Debug)]
	pub enum StartHTLTransactionError {
		InvalidTransaction {
			description("Invalid transaction")
		}
	}
}
