use std::{
    collections::{hash_map::Iter, HashMap, HashSet},
    error::Error,
    rc::Rc,
};
use serde_derive::{Serialize, Deserialize};
use crate::{
    history::ledger::SingleLedgerContexts,
    ledger::LedgerIdRc,
    operations::LedgerOperation,
    types::{AgentAddressRc, HashStringRc},
};

pub type HashedTimelockPreimage = String;
pub type LedgerIds = HashSet<LedgerIdRc>;
pub type LedgerOperations = Vec<LedgerOperation>;
pub type SequenceNumbers = HashMap<LedgerIdRc, u64>;
pub type TransactionAgent = AgentAddressRc;
pub type TransactionId = HashStringRc;
pub type TransactionMetadata = HashStringRc;

pub type TransactionEffectKey = (&'static str, String);
pub type TransactionEffects = HashMap<TransactionEffectKey, String>;

/**
 * Provides the proof justifying the failing or fulfilling of the hashed
 * timelock transaction.
 */
#[derive(Serialize, Deserialize, Clone, Debug)]
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
    Fulfilled(HashedTimelockPreimage),
}

/**
 * Enum of the possible reasons and proofs for failing a hashed timelock
 * transaction.
 */
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "reason", content = "proof")]
pub enum HashedTimeLockFailureReason {
    /// No path exists from this agent to the destination.
    NoPath,

    ///
    ExceedMaxHops,

    ///
    Timeout(String),
}

/**
 * Provides access to the set of `SingleLedgerContexts`s and any effects a
 * transaction may have.
 */
pub trait TransactionContext {
    /**
     * Determines if the `TransactionContext` already contains the given set of
     * `LedgerState`s.
     *
     * Useful during history reconstruction when deciding whether or not to
     * skip validation and application of a given operation (since it's
     * validation and application won't be relevant to the newest transaction).
     */
    fn has_ledger(&self, ledger_id: &LedgerIdRc) -> bool;

    /**
     * Determines if the `TransactionContext` contains all
     * `SingleLedgerContexts` necessary to validate a given transaction.
     *
     * Useful during validation and application of a new transaction.
     */
    fn has_all_ledgers(&self, ids: &LedgerIds) -> bool;

    /**
     * Retrieves the `SingleLedgerContexts` for a given `LedgerIdRc`.
     */
    fn ledger_context(
        &self,
        ledger_id: &LedgerIdRc
    ) -> Option<&SingleLedgerContexts>;

    /**
     * Returns an iterator over the containing ledgers' `SingleLedgerContexts`.
     */
    fn ledger_iter(&self) -> Iter<LedgerIdRc, SingleLedgerContexts>;

    fn effects(&self) -> &TransactionEffects;
    fn mut_effects(&mut self) -> &mut TransactionEffects;
}

/**
 * Validation and application of changes to a set of ledgers.
 */
pub trait Transaction {
    type Error;

    /**
     * Retrieves the `TransactionId` of a given transaction.
     */
    fn id(&self) -> TransactionId;

    /**
     * Retrives a reference to the given transaction's affected ledgers and
     * their new (upon success) sequence numbers.
     */
    fn seq_nos(&self) -> &SequenceNumbers;

    /**
     * Retrieves a reference to the given transaction's underlying
     * `LedgerOperations` vector.
     */
    fn operations(&self) -> Option<&LedgerOperations>;

    /**
     *
     */
    fn mut_validate_and_apply<C: TransactionContext>(
        &self,
        context: C
    ) -> Result<C, Self::Error>;

    /**
     * Retrives the set of all `LedgerIdRc`s explicitly listed alongside their
     * new sequence numbers in a given transaction.
     */
    fn seq_ledger_ids(&self) -> LedgerIds {
        self.seq_nos()
            .keys()
            .fold(HashSet::new(), |mut ids, id| {
                ids.insert(Rc::clone(id));
                ids
            })
    }

    /**
     * Retrives the set of all `LedgerIdRc`s explicitly listed within the given
     * transaction's list of contained `LedgerOperation`s.
     */
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

    /**
     * Retrives an `Option` of the set of all `LedgerIds` required for
     * validation and  application of this transaction, or `None` if there's
     * a mismatch in ledger requirements between the specified sequence
     * numbers and listed operations.
     */
    fn required_ledger_ids(&self) -> Option<LedgerIds> {
        let seq_ledger_ids = self.seq_ledger_ids();
        let op_ledger_ids = self.operation_ledger_ids();
        match seq_ledger_ids == op_ledger_ids {
            true => Some(seq_ledger_ids),
            false => None,
        }
    }
}
