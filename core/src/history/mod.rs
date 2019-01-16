//! # General Idea
//!
//! each transaction requires a "powerset" for each ledger
//! by applying the newest transaction to the powerset of each affected ledger, we know that regardless of the success/failure of previous htl transactions, that this transaction can be applied/fulfilled later
//! - starting with a blank, "up-to-date" ledger (i.e. the most recent before application of any htls)
//! - applying each transaction (in order) since ledger seq no:
//! 	- forking upon every htl (one revoked, one fulfilled one for each pending transactions)
//! 	- applying any and all basic transactions (in order) to fork of ledger of the powerset
//!
//! - (when replaying history) ignoring any operations that dont apply to any of our current view's ledgers
//!
//! ## Algorithm for creating tree of ledger history (given a new tx):
//! - Side Effects (via Holochain API)
//! 	- list_operation_ledger_ids from new tx
//! 	- for each unique ledger_id:
//! 		- get (from storage) latest version of that ledger that doesnt include any htls
//! 		- get (from storage) all transactions for this ledger since it's sequence number
//!
//! - MultiLedgerState Generation (`new(tx)`)
//! 	- inits a HashMap of trees (one for each unique ledger in tx)
//! 	- inits HashMap<ID, MultiLedgerTransaction>
//!
//! - MultiLedgerState addition (`add_ledger(ledger, tx_list)`)
//! 	- moves new (unseen) txs to the hashmap
//! 	-
//!
//! - LedgerOperationHistory Generation (`new(ledger, Vec<&Tx>)` method):
//! 	- adds ledger to the tree,
//! 	-
//! 	- ?? filter out operations that don't apply to that ledger

pub mod ledger;
pub mod multiledger;
