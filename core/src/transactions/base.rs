use std::{
    collections::{hash_map::Iter, HashMap, HashSet},
    error::Error,
    rc::Rc,
};
use serde_derive::{Serialize, Deserialize};
use crate::{
    history::operation::OperationHistory,
    ledger::LedgerId,
    operations::{*, base::*},
};

pub type LedgerIds = HashSet<LedgerId>;
pub type Operations = Vec<LedgerOperation>;
pub type SequenceNumbers = HashMap<LedgerId, u64>;
pub type TransactionId = Rc<String>;
pub type TransactionAgent = Rc<String>;

pub type TransactionEffectKey = (&'static str, String);
pub type TransactionEffects = HashMap<TransactionEffectKey, String>;

/**
 * Provides access to the set of `OperationHistory`s and any effects a
 * transaction may have.
 */
pub trait MultiLedgerHistory {
    /**
     * Determines if the `MultiLedgerHistory` already contains a given
     * `OperationHistory`.
     *
     * Useful during history reconstruction when deciding whether or not to
     * skip validation and application of a given operation (since it's
     * validation and application won't be relevant to the newest transaction).
     */
    fn has_history(&self, ledger_id: &LedgerId) -> bool;

    /**
     * Determines if the `MultiLedgerHistory` contains all `OperationHistory`s
     * necessary to validate a given transaction.
     *
     * Useful during validation and application of a new transaction.
     */
    fn has_all_histories(&self, ids: &LedgerIds) -> bool;

    /**
     * Retrieves the `OperationHistory` for a given `LedgerId`.
     */
    fn get(&self, ledger_id: &LedgerId) -> Option<&OperationHistory>;

    /**
     * Returns an iterator over the containing ledgers' `OperationHistory`s.
     */
    fn iter(&self) -> Iter<LedgerId, OperationHistory>;

    fn effects(&self) -> &TransactionEffects;
    fn mut_effects(&mut self) -> &mut TransactionEffects;
}

/**
 * Provides the proof justifying the failing or fulfilling of the hashed
 * timelock transaction.
 */
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "payload")]
pub enum HashedTimeLockProof {
    /**
     * Contains a reason for HTL transaction failure.
     *
     * Could (eventually) be a VDF proof, signature of timestamp, etc.
     */
    Failed(HashedTimeLockFailureReason),

    /**
     * Contains the preimage necessary to fulfill an HTL transaction.
     */
    Fulfilled(String),
}

/**
 * Enum of the possible reasons and proofs for failing a hashed timelock
 * transaction.
 */
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "reason", content = "proof")]
pub enum HashedTimeLockFailureReason {
    /// No path exists from this agent to the destination.
    NoPath,

    ///
    ExceedMaxHops,

    ///
    Timeout(String),
}

/// TODO: is this necessary??
#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionMetadata {
    app_hash: String,
    entry_hash: String, // TODO: entry_id_anchor instead?
}

/**
 * Validation and application of changes to a set of ledgers.
 */
pub trait Transaction<TxError: Error> {
    /// Retrieves the `TransactionId` of a given transaction.
    fn id(&self) -> TransactionId;

    /// Retrives a reference to the given transaction's affected ledgers and
    /// their new (upon success) sequence numbers.
    fn seq_nos(&self) -> &SequenceNumbers;

    /// Retrieves a reference to the given transaction's underlying
    /// `Operations` vector.
    fn operations(&self) -> Option<&Operations>;

    /// Retrives the set of all `LedgerId`s explicitly listed alongside their
    /// new sequence numbers in a given transaction.
    fn seq_ledger_ids(&self) -> LedgerIds {
        self.seq_nos()
            .keys()
            .fold(HashSet::new(), |mut ids, id| {
                ids.insert(Rc::clone(id));
                ids
            })
    }

    /// Retrives the set of all `LedgerId`s explicitly listed within the given
    /// transaction's list of contained `LedgerOperation`s.
    fn operation_ledger_ids(&self) -> LedgerIds {
        match self.operations() {
            None => HashSet::new(),
            Some(ops) => ops
                .iter()
                .fold(HashSet::new(), |mut ids, op| {
                    ids.insert(op.ledger_id());
                    ids
                })
        }
    }

    /// Retrives an `Option` of the set of all `LedgerIds` required for
    /// validation and  application of this transaction, or `None` if there's
    /// a mismatch in ledger requirements between the specified sequence
    /// numbers and listed operations.
    fn required_ledger_ids(&self) -> Option<LedgerIds> {
        let seq_ledger_ids = self.seq_ledger_ids();
        let op_ledger_ids = self.operation_ledger_ids();
        match seq_ledger_ids == op_ledger_ids {
            true => Some(seq_ledger_ids),
            false => None,
        }
    }
}
