use std::collections::HashMap;
use quick_error::quick_error;
use serde_derive::{Serialize, Deserialize};
use self::{
    base::*,
    basic::{Error as BasicTransactionError, *},
    start_htl::{Error as StartHTLTransactionError, *},
    end_htl::{Error as EndHTLTransactionError, *},
};

pub mod base;
pub mod basic;
pub mod start_htl;
pub mod end_htl;

/// Stores all transactions relevant to multiple ledgers' histories
pub type TransactionMap = HashMap<TransactionId, MultiLedgerTransaction>;

pub const ENTRY_TYPE: &'static str = "multiledger_transaction";

/**
 *
 */
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum MultiLedgerTransaction {
    Basic(BasicTransaction),
    StartHTL(StartHTLTransaction),
    EndHTL(EndHTLTransaction),
}

impl MultiLedgerTransaction {
    /// Validates and applies the transaction and it's operations against the
    /// ledgers available in `MultiLedgerHistory`
    pub fn mut_validate_and_apply<H: MultiLedgerHistory>(
        &self,
        transactions: &TransactionMap,
        multiledger_history: H,
    ) -> Result<H, Error> {
        self.validate_seq_no(&multiledger_history)?;

        match self {
            MultiLedgerTransaction::Basic(tx) => tx
                .mut_validate_and_apply(multiledger_history)
                .map_err(Error::BasicTransactionError),
            MultiLedgerTransaction::StartHTL(tx) => tx
                .mut_validate_and_apply(multiledger_history)
                .map_err(Error::StartHTLTransactionError),
            MultiLedgerTransaction::EndHTL(tx) => transactions
                .get(&tx.start_htl_id())
                .and_then(|start_htl| start_htl.unwrap_start_htl())
                .ok_or(Error::InvalidStartHTLError)
                .and_then(|start_htl| tx
                    .mut_validate_and_apply(start_htl, multiledger_history)
                    .map_err(Error::EndHTLTransactionError)
                ),
        }
    }

    /// Validates and applies the transaction and it's operations against the
    /// ledgers available in `MultiLedgerHistory`, but also guarantees that all
    /// ledgers required by the transaction are available.
    pub fn mut_validate_and_apply_new<H: MultiLedgerHistory>(
        &self,
        transactions: &TransactionMap,
        multiledger_history: H,
    ) -> Result<H, Error> {
        // ensure no ops require ledgers not in multiledger_history
        self.required_ledger_ids(transactions)
            .ok_or(Error::InvalidEndHTLError)
            .and_then(|ref required_ledger_ids| {
                if multiledger_history.has_all_histories(required_ledger_ids) {
                    self.mut_validate_and_apply(transactions, multiledger_history)
                } else {
                    Err(Error::InvalidEndHTLError)
                }
            })
    }

    /// Checks that the transaction's ledger sequence number bumps are valid
    /// against multi ledger history.
    ///
    /// NOTE: only checks against `MultiLedgerState`s present in
    /// `multiledger_history`
    fn validate_seq_no<H: MultiLedgerHistory>(
        &self,
        multiledger_history: &H,
    ) -> Result<(), Error> {
        self.seq_nos()
            .iter()
            .filter(|(id, _)| multiledger_history.has_history(id))
            .map(|(ledger_id, tx_seq_no)| multiledger_history
                .get(ledger_id)
                .and_then(|op_history| op_history.current_seq_no())
                .map(|ledger_seq_no| (ledger_seq_no, tx_seq_no))
            )
            .fold(Ok(()), |seq_nos_are_valid, seq_nos| seq_nos_are_valid
                .and_then(|_| seq_nos.ok_or(Error::InvalidSequenceNumberError))
                .and_then(|(ledger_seq_no, &tx_seq_no)| {
                    let new_seq_no = ledger_seq_no + 1;
                    if tx_seq_no.lt(&new_seq_no) {
                        Err(Error::RepeatedSequenceNumberError)
                    } else if tx_seq_no.gt(&new_seq_no) {
                        Err(Error::SkippedSequenceNumberError)
                    } else {
                        Ok(())
                    }
                })
            )
    }

    fn required_ledger_ids(
        &self,
        transactions: &TransactionMap,
    ) -> Option<LedgerIds> {
        match self {
            MultiLedgerTransaction::Basic(tx) => tx.required_ledger_ids(),
            MultiLedgerTransaction::StartHTL(tx) => tx.required_ledger_ids(),
            MultiLedgerTransaction::EndHTL(tx) => transactions
                .get(&tx.start_htl_id())
                .and_then(|start_htl_mlt| start_htl_mlt.unwrap_start_htl())
                .and_then(|start_htl| tx.required_ledger_ids(start_htl)),
        }
    }

    /// Retrieve the nested `StartHTLTransaction` from the container
    /// `MultiLedgerTransaction`, if possible
    fn unwrap_start_htl(&self) -> Option<&StartHTLTransaction> {
        match self {
            MultiLedgerTransaction::StartHTL(tx) => Some(tx),
            _ => None,
        }
    }
}

impl Transaction<Error> for MultiLedgerTransaction {
    fn id(&self) -> TransactionId {
        match self {
            MultiLedgerTransaction::Basic(tx) => tx.id(),
            MultiLedgerTransaction::StartHTL(tx) => tx.id(),
            MultiLedgerTransaction::EndHTL(tx) => tx.id(),
        }
    }

    fn seq_nos(&self) -> &SequenceNumbers {
        match self {
            MultiLedgerTransaction::Basic(tx) => tx.seq_nos(),
            MultiLedgerTransaction::StartHTL(tx) => tx.seq_nos(),
            MultiLedgerTransaction::EndHTL(tx) => tx.seq_nos(),
        }
    }

    fn operations(&self) -> Option<&Operations> {
        match self {
            MultiLedgerTransaction::Basic(tx) => tx.operations(),
            MultiLedgerTransaction::StartHTL(tx) => tx.operations(),
            MultiLedgerTransaction::EndHTL(tx) => tx.operations(),
        }
    }

    fn operation_ledger_ids(&self) -> LedgerIds {
        match self {
            MultiLedgerTransaction::Basic(tx) =>
                tx.operation_ledger_ids(),
            MultiLedgerTransaction::StartHTL(tx) =>
                tx.operation_ledger_ids(),
            MultiLedgerTransaction::EndHTL(tx) =>
                tx.operation_ledger_ids(),
        }
    }

    fn required_ledger_ids(&self) -> Option<LedgerIds> {
        panic!("Use `required_ledger_ids(&self, txs: &TransactionMap)");
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        NonExistantStartHTLError {
            description("StartHTL transaction (referred to by an EndHTL transaction) does not exist")
        }
        InvalidStartHTLError {
            description("StartHTL transaction is invalid")
        }
        InvalidEndHTLError {
            description("EndHTL transaction is invalid")
        }
        InvalidSequenceNumberError {
            description("Transaction conflicts with current ledger sequence number")
        }
        RepeatedSequenceNumberError {
            description("Transaction requires reusing a ledger sequence number")
        }
        SkippedSequenceNumberError {
            description("Transaction requires skipping a ledger sequence number; some transactions may not have been applied")
        }
        BasicTransactionError(err: BasicTransactionError) {
            description(err.description())
        }
        StartHTLTransactionError(err: StartHTLTransactionError) {
            description(err.description())
        }
        EndHTLTransactionError(err: EndHTLTransactionError) {
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
