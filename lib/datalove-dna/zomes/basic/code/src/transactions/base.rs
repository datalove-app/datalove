use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::rc::Rc;
use history::operation::OperationHistory;
use operations::*;
use operations::base::*;
use types::*;

pub type LedgerIds = HashSet<Rc<Hash>>;
pub type Operations = Vec<LedgerOperation>;
pub type SequenceNumbers = HashMap<Rc<Hash>, u64>;

/// Stores the side effects of applying a transaction, i.e.:
/// - changes to multiple ledgers
/// - side effects of relevance to future operations
pub type MultiLedgerState = HashMap<Hash, OperationHistory>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "status", content = "payload")]
pub enum HashedTimeLockProof {
	// contains a reason (i.e. VDF proof, signature of timestamp, etc)
	Failed(HashedTimeLockFailureReason),
	// contains the preimage
	Fulfilled(Hash),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "reason", content = "proof")]
pub enum HashedTimeLockFailureReason {
	Timeout(String),
	NoPath,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionMetadata {
	app_hash: Hash,
	entry_hash: Hash,
}

pub trait Transaction<'a, TxError: Error> {
	fn id(&self) -> Rc<Hash>;

	fn operations(&self) -> &Operations;

	fn seq_nos(&'a self) -> &SequenceNumbers;

	// fn validate_and_apply(
	// 	&self,
	// 	multiledger_state: MultiLedgerState,
	// ) -> Result<MultiLedgerState, TxError> {
	// 	Err(MultiLedgerTransactionError::Basic)
	// }

	// DEFAULTS

	fn is_valid_seq_no(&self, multiledger_state: &MultiLedgerState) -> bool {
		false
	}

	// TODO: move core logic to... somewhere else
	// fn is_valid_seq_no(
	// 	&'a self,
	// 	starting_ledger: &'a Ledger,
	// 	previous_transactions: &Vec<impl Transaction<'a, TxError>>,
	// ) -> bool {
	// 	let initial_seq_no = starting_ledger.sequence_number();
	// 	let new_seq_no = self.seq_nos().get(starting_ledger.id());
	// 	if Option::is_none(&new_seq_no) {
	// 		return false;
	// 	}

	// 	let latest_seq_no = previous_transactions
	// 		.iter()
	// 		.fold(initial_seq_no, |current_seq_no, tx| {
	// 			match tx.seq_nos().get(starting_ledger.id()) {
	// 				Some(&seq_no) if seq_no == current_seq_no + 1 => seq_no,
	// 				_ => current_seq_no,
	// 			}
	// 		});

	// 	return latest_seq_no + 1 == *new_seq_no.unwrap();
	// }

	fn seq_ledger_ids(&'a self) -> LedgerIds {
		self.seq_nos()
			.keys()
			.fold(HashSet::new(), |mut ids, id| {
				ids.insert(Rc::clone(id));
				ids
			})
	}

	fn operation_ledger_ids(&'a self) -> LedgerIds {
		self.operations()
			.iter()
			.fold(HashSet::new(), |mut ids, op| {
				ids.insert(Rc::new(op.ledger_id().clone()));
				ids
			})
	}

	fn required_ledger_ids(&'a self) -> Option<LedgerIds> {
		let seq_ledger_ids = self.seq_ledger_ids();
		let op_ledger_ids = self.operation_ledger_ids();
		match seq_ledger_ids == op_ledger_ids {
			true => Some(seq_ledger_ids),
			false => None,
		}
	}
}

quick_error! {
	#[derive(Debug)]
	pub enum TransactionError {
		MismatchedLedgerIdsError {}
	}
}
