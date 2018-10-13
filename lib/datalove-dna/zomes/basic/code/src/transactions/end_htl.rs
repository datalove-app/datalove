use std::collections::{HashSet};
use std::rc::Rc;
use types::*;
use super::base::*;
use super::start_htl::StartHTLTransaction;

lazy_static! {
	static ref EMPTY_OP_LIST: Operations = Vec::new();
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndHTLTransaction {
	id: Rc<Hash>,
	sender: Rc<Hash>,
	seq_nos: SequenceNumbers,
	start_tx_hash: Rc<Hash>,
	proof: HashedTimeLockProof,
}

impl EndHTLTransaction {
	pub fn start_htl_hash(&self) -> Rc<Hash> { Rc::clone(&self.start_tx_hash) }

	pub fn validate_and_apply(
		&self,
		start_htl: &StartHTLTransaction,
		multiledger_state: MultiLedgerState,
	) -> Result<MultiLedgerState, EndHTLTransactionError> {
		// ensure all seq_ledger_ids in start_htl are listed in &self
		// ensure this txn's seq_no is one greater than seq_no in ledger

		Err(EndHTLTransactionError::InvalidTransaction)
	}

	pub fn required_ledger_ids(
		&self,
		start_htl: &StartHTLTransaction,
	) -> Option<LedgerIds> {
		let start_htl_ledger_ids = start_htl.required_ledger_ids();
		let end_htl_ledger_ids = self.seq_ledger_ids();
		match start_htl_ledger_ids.as_ref() {
			Some(start_ids) if end_htl_ledger_ids.eq(start_ids) => {
				Some(end_htl_ledger_ids)
			},
			_ => None,
		}
	}
}

impl<'a> Transaction<'a, EndHTLTransactionError> for EndHTLTransaction {
	fn id(&self) -> Rc<Hash> { Rc::clone(&self.id) }
	fn operations(&self) -> &Operations { &EMPTY_OP_LIST }
	fn seq_nos(&self) -> &SequenceNumbers { &self.seq_nos }

	fn operation_ledger_ids(&'a self) -> LedgerIds { HashSet::new() }

	// fn validate(
	// 	&self,
	// ) -> Result<&Self, EndHTLTransactionError> {
	// 	Ok(self)
	// }

	// fn mut_apply(&self, ledgers: &mut Vec<Ledger>) {}
}

quick_error! {
	#[derive(Debug)]
	pub enum EndHTLTransactionError {
		InvalidTransaction {
			description("Invalid transaction")
		}
	}
}
