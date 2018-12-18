use std::convert::TryFrom;
use quick_error::quick_error;
use serde_derive::{Serialize, Deserialize};
use self::{
    base::*,
    basic::{BasicTransaction, Error as BasicTransactionError},
    start_htl::{StartHTLTransaction, Error as StartHTLTransactionError},
    end_htl::{EndHTLTransaction, Error as EndHTLTransactionError},
};

pub mod base;
pub mod basic;
pub mod start_htl;
pub mod end_htl;

/**
 * Stores all transactions relevant to multiple ledgers' histories
 */
pub trait TransactionsMap {
    fn get(&self, id: &TransactionId) ->
        Option<&MultiLedgerTransaction>;
    fn get_basic(&self, id: &TransactionId) ->
        Option<&BasicTransaction>;
    fn get_start_htl(&self, id: &TransactionId) ->
        Option<&StartHTLTransaction>;
    fn get_end_htl(&self, id: &TransactionId) ->
        Option<&EndHTLTransaction>;
}

pub const ENTRY_TYPE: &'static str = "multiledger_transaction";

/**
 *
 */
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum MultiLedgerTransaction {
    Basic(BasicTransaction),
    StartHTL(StartHTLTransaction),
    EndHTL(EndHTLTransaction),
}

impl MultiLedgerTransaction {
    pub fn id(&self) -> TransactionId {
        match self {
            MultiLedgerTransaction::Basic(tx) => tx.id(),
            MultiLedgerTransaction::StartHTL(tx) => tx.id(),
            MultiLedgerTransaction::EndHTL(tx) => tx.id(),
        }
    }

    /**
     * Validates and applies the transaction and it's operations against the
     * ledgers available in `MultiLedgerState`, but also guarantees that all
     * ledgers required by the transaction are available.
     */
    pub fn mut_validate_and_apply_new<C: TransactionContext + TransactionsMap>(
        &self,
        context: C,
    ) -> Result<C, Error> {
        // ensure no ops require ledgers not in multiledger_state
        self.required_ledger_ids(&context)
            .ok_or(Error::InvalidEndHTLError)
            .and_then(|ref required_ledger_ids| {
                if context.has_all_ledgers(required_ledger_ids) {
                    self.mut_validate_and_apply(context)
                } else {
                    Err(Error::InvalidEndHTLError)
                }
            })
    }

    /**
     * Validates and applies the transaction and it's operations against the
     * ledgers available in `MultiLedgerState`
     */
    pub fn mut_validate_and_apply<C: TransactionContext + TransactionsMap>(
        &self,
        context: C,
    ) -> Result<C, Error> {
        self.validate_seq_no(&context)?;

        match self {
            MultiLedgerTransaction::Basic(tx) => tx
                .mut_validate_and_apply(context)
                .map_err(Error::BasicTransactionError),
            MultiLedgerTransaction::StartHTL(tx) => tx
                .mut_validate_and_apply(context)
                .map_err(Error::StartHTLTransactionError),
            MultiLedgerTransaction::EndHTL(tx) => context
                .get_start_htl(&tx.start_htl_id())
                .ok_or(Error::InvalidStartHTLError)
                .map(|start_htl| start_htl.clone())
                .and_then(|start_htl| tx
                    // TODO: avoid cloning if possible
                    .mut_validate_and_apply(&start_htl.clone(), context)
                    .map_err(Error::EndHTLTransactionError)
                ),
        }
    }

    /**
     * Checks that the transaction's ledger sequence number bumps are valid
     * against multi ledger history.
     *
     * NOTE: only checks against `MultiLedgerState`s present in
     * `multiledger_state`
     */
    fn validate_seq_no(
        &self,
        context: &TransactionContext,
    ) -> Result<(), Error> {
        self.seq_nos()
            .iter()
            .filter(|(id, _)| context.has_ledger(id))
            .map(|(ledger_id, tx_seq_no)| context
                .ledger_context(ledger_id)
                .and_then(|ledger_context| ledger_context.current_seq_no())
                .map(|ledger_seq_no| (ledger_seq_no, tx_seq_no))
            )
            .fold(Ok(()), |seq_nos_are_valid, seq_nos| seq_nos_are_valid
                .and_then(|_| seq_nos.ok_or(Error::InvalidSequenceNumberError))
                .and_then(|(ledger_seq_no, &tx_seq_no)| {
                    let expected_seq_no = ledger_seq_no + 1;
                    if tx_seq_no.lt(&expected_seq_no) {
                        Err(Error::RepeatedSequenceNumberError)
                    } else if tx_seq_no.gt(&expected_seq_no) {
                        Err(Error::SkippedSequenceNumberError)
                    } else {
                        Ok(())
                    }
                })
            )
    }
}

impl MultiLedgerTransaction {
    fn seq_nos(&self) -> &SequenceNumbers {
        match self {
            MultiLedgerTransaction::Basic(tx) => tx.seq_nos(),
            MultiLedgerTransaction::StartHTL(tx) => tx.seq_nos(),
            MultiLedgerTransaction::EndHTL(tx) => tx.seq_nos(),
        }
    }

    fn operations(&self) -> Option<&LedgerOperations> {
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

    fn required_ledger_ids<C: TransactionContext + TransactionsMap>(
        &self,
        context: &C,
    ) -> Option<LedgerIds> {
        match self {
            MultiLedgerTransaction::Basic(tx) => tx.required_ledger_ids(),
            MultiLedgerTransaction::StartHTL(tx) => tx.required_ledger_ids(),
            MultiLedgerTransaction::EndHTL(tx) => context
                .get_start_htl(&tx.start_htl_id())
                .and_then(|start_htl| tx.required_ledger_ids(start_htl)),
        }
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        NonExistantStartHTLError {
            description("StartHTL transaction (referred to by an EndHTL transaction) does not exist")
        }
        InvalidBasicHTLError {
            description("Basic transaction is invalid")
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
