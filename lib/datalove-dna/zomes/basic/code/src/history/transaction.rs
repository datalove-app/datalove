use std::collections::{HashMap, HashSet};
// use std::error::Error;
use std::rc::Rc;
// use id_tree::Tree;
use ledger::*;
use transactions::*;
use transactions::base::*;
use types::*;

pub type TransactionOrder<'a> = Vec<&'a Hash>;
pub type TransactionOrders<'a> = HashMap<&'a Hash, TransactionOrder<'a>>;

pub struct TransactionHistory<'a> {
	// root_transaction: MultiLedgerTransaction,
	// transaction_set: HashMap<Hash, usize>,
	// pending_htls: Vec<Hash>,

	// set of all affected ledgers of interest
	ledger_ids: LedgerIds,
	// a set of all ledgers and their potential histories
	ledgers: MultiLedgerState,
	// a list of all transactions
	transactions: TransactionMap,
	// an ordering of transactions for each ledger
	transaction_orders: TransactionOrders<'a>,
}

// PUBLIC METHODS
impl<'a> TransactionHistory<'a> {
	// initializes a history around a new transaction
	pub fn new(transaction: MultiLedgerTransaction) -> Result<Self, ()> {
		// validate new transaction against itself
			// i.e. does seq_ledger_ids and operation_ledger_ids match?

		match transaction.required_ledger_ids() {
			None => Err(()),
			Some(ledger_ids) => {
				let mut tx_map = HashMap::new();
				tx_map.insert(transaction.id(), transaction);

				Ok(TransactionHistory {
					ledger_ids,
					ledgers: HashMap::new(),
					transactions: tx_map,
					transaction_orders: HashMap::new(),
				})
			}
		}
	}

	// creates a new LedgerOperationHistory, applying each transaction
	pub fn mut_apply_ledger(
		&'a mut self,
		ledger: Ledger,
		transactions: Vec<MultiLedgerTransaction>
	) -> &'a Self {
		// validates new transaction against to-be-added ledger
			// i.e. if basic, are we the owner? etc
		// validates new transaction against transaction history
			// are there gaps in seq_no, and if not, does it end in one less than current transactions seq_no?
		// if valid
			// call LedgerOperationHistory::new; if successful, adds it to TransactionHistory

		self
	}
}

// PRIVATE METHODS
impl<'a> TransactionHistory<'a> {
	fn validate_transaction(
		&self,
		transaction: &MultiLedgerTransaction
	) -> Result<&Self, ()> {
		match transaction {
			MultiLedgerTransaction::Basic(tx) =>
				self.validate_basic(),
			MultiLedgerTransaction::StartHTL(tx) =>
				self.validate_start_htl(),
			MultiLedgerTransaction::EndHTL(tx) =>
				self.validate_end_htl(),
		}
	}

	fn mut_apply_transaction(
		&mut self,
		transaction: &MultiLedgerTransaction,
	) -> Result<&mut Self, ()> {
		match transaction {
			MultiLedgerTransaction::Basic(tx) =>
				self.mut_apply_basic(),
			MultiLedgerTransaction::StartHTL(tx) =>
				self.mut_apply_start_htl(),
			MultiLedgerTransaction::EndHTL(tx) =>
				self.mut_apply_end_htl(),
		}
	}

	fn validate_basic(
		&self,
		// transaction: &basic::BasicTransaction,
	) -> Result<&Self, ()> {
		// self.validate_ops(transaction.operations())?;

		Ok(self)
	}

	fn validate_start_htl(
		&self,
		// transaction: &start_htl::StartHTLTransaction
	) -> Result<&Self, ()> {
		Ok(self)
	}

	fn validate_end_htl(
		&self,
		// transaction: &end_htl::EndHTLTransaction
	) -> Result<&Self, ()> {
		Ok(self)
	}

	fn mut_apply_basic(
		&mut self,
		// transaction: &basic::BasicTransaction,
	) -> Result<&mut Self, ()> {
		// for each existing ledger in self
			// validate transaction against ledger
			// for each operation for this ledger
				// validate the operation against ledger (and history?)
			// if both transaction and all operations are valid
				// apply operations to ledger

		Ok(self)
	}

	fn mut_apply_start_htl(
		&mut self,
		// transaction: &start_htl::StartHTLTransaction
	) -> Result<&mut Self, ()> {
		// for each existing ledger in self
			// validate transaction against ledger
			// for each operation for this ledger
				// validate the operation against ledger (and history?)
			// if both transaction and all operations are valid
				// clone this ledger
				// apply operations to ledger clone

		Ok(self)
	}

	fn mut_apply_end_htl(
		&mut self,
		// transaction: &end_htl::EndHTLTransaction
	) -> Result<&mut Self, ()> {
		// for each existing ledger in self
			// validate transaction against ledger
				// i.e. ensure that sequence numbers match up
			// transaction is valid
				// delete ledger clones that don't include the successful start_htl

		Ok(self)
	}

	////////////////////////////////////////////////////////////////////////////
	////////////////////////////////////////////////////////////////////////////
	////////////////////////////////////////////////////////////////////////////

	// validate the current transaction against the entire TransactionHistory
	pub fn validate(&self) -> Result<&Self, ()> {
		Err(())
	}

	//
	pub fn mut_apply(&mut self) -> Result<&mut Self, ()> {
		Err(())
	}
}
