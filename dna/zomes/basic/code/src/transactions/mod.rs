use std::collections::HashMap;
use std::rc::Rc;
use types::*;
use self::base::*;
use self::basic::*;
use self::start_htl::*;
use self::end_htl::*;
use self::MultiLedgerTransactionError as Error;

pub mod base;
mod basic;
mod start_htl;
mod end_htl;

/// Stores all transactions relevant to multiple ledgers' histories
pub type TransactionMap = HashMap<Rc<Hash>, MultiLedgerTransaction>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum MultiLedgerTransaction {
	Basic(BasicTransaction),
	StartHTL(StartHTLTransaction),
	EndHTL(EndHTLTransaction),
}

impl MultiLedgerTransaction {
	pub fn validate_and_apply(
		&self,
		ledger_ids: &LedgerIds,
		transactions: &TransactionMap,
		multiledger_state: MultiLedgerState,
	) -> Result<MultiLedgerState, Error> {
		match self {
			MultiLedgerTransaction::Basic(tx) => tx
				.validate_and_apply(ledger_ids, multiledger_state)
				.map_err(Error::Basic),
			MultiLedgerTransaction::StartHTL(tx) => tx
				.validate_and_apply(ledger_ids, multiledger_state)
				.map_err(Error::StartHTL),
			MultiLedgerTransaction::EndHTL(tx) => {
				let start_htl_hash = tx.start_htl_hash();
				match transactions.get(&start_htl_hash) {
					None => Err(Error::NonExistantStartHTLError),
					Some(start_htl) => {
						let start_htl = start_htl.unwrap_start_htl()?;
						tx
							.validate_and_apply(start_htl, multiledger_state)
							.map_err(Error::EndHTL)
					},
				}

			},
		}
	}

	fn unwrap_start_htl(&self) -> Result<&StartHTLTransaction, Error> {
		match self {
			MultiLedgerTransaction::StartHTL(tx) => Ok(tx),
			_ => Err(Error::InvalidStartHTLError),
		}
	}
}

impl<'a> Transaction<'a, Error> for MultiLedgerTransaction {
	fn id(&self) -> Rc<Hash> {
		match self {
			MultiLedgerTransaction::Basic(tx) => tx.id(),
			MultiLedgerTransaction::StartHTL(tx) => tx.id(),
			MultiLedgerTransaction::EndHTL(tx) => tx.id(),
		}
	}

	fn operations(&self) -> &Operations {
		match self {
			MultiLedgerTransaction::Basic(tx) => tx.operations(),
			MultiLedgerTransaction::StartHTL(tx) => tx.operations(),
			MultiLedgerTransaction::EndHTL(tx) => tx.operations(),
		}
	}

	fn seq_nos(&self) -> &SequenceNumbers {
		match *self {
			MultiLedgerTransaction::Basic(ref tx) => tx.seq_nos(),
			MultiLedgerTransaction::StartHTL(ref tx) => tx.seq_nos(),
			MultiLedgerTransaction::EndHTL(ref tx) => tx.seq_nos(),
		}
	}

	// lists all ids of ledgers required by all operations
	fn operation_ledger_ids(&'a self) -> LedgerIds {
		match self {
			MultiLedgerTransaction::Basic(tx) =>
				tx.operation_ledger_ids(),
			MultiLedgerTransaction::StartHTL(tx) =>
				tx.operation_ledger_ids(),
			MultiLedgerTransaction::EndHTL(tx) =>
				tx.operation_ledger_ids(),
		}
	}

	// fn validate(&self, ledger: &Ledger) -> Result<&Self, String> {}

	// fn mut_apply(&self, mut_ledger: &mut Ledger) -> Result<&mut Ledger, String> {}
}

quick_error! {
	#[derive(Debug)]
	pub enum MultiLedgerTransactionError {
		InvalidStartHTLError {
			description("StartHTL transaction is invalid")
		}
		NonExistantStartHTLError {
			description("StartHTL transaction (referred to by an EndHTL transaction) does not exist")
		}
		Basic(err: BasicTransactionError) {
			description(err.description())
		}
		StartHTL(err: StartHTLTransactionError) {
			description(err.description())
		}
		EndHTL(err: EndHTLTransactionError) {
			description(err.description())
		}
	}
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
		assert!(true);
    }
}
