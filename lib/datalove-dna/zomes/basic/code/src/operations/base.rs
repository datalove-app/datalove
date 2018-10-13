use std::collections::HashMap;
use std::error::Error;
use ledger::*;
use types::*;

pub type LedgerEffectKey = (&'static str, String);
pub type LedgerEffects = HashMap<LedgerEffectKey, String>;

/// Stores the side effects of applying an operation, i.e.:
/// - changes to the ledger
/// - side effects of relevance to future operations
pub struct LedgerState {
	ledger: Ledger,
	effects: LedgerEffects,
}

impl LedgerState {
	pub fn new(ledger: Ledger) -> Self {
		LedgerState { ledger, effects: HashMap::new() }
	}

	pub fn ledger(&self) -> &Ledger { &self.ledger }
	pub fn effects(&self) -> &LedgerEffects { &self.effects }

	pub fn mut_ledger(&mut self) -> &mut Ledger { &mut self.ledger }
	pub fn mut_effects(&mut self) -> &mut LedgerEffects { &mut self.effects }
}

pub trait Operation<'a, OpError: Error> {
	fn ledger_id(&self) -> &Hash;

	fn is_htl_only(&self) -> bool { false }

	// Determines if the operation can be applied to a given ledger
	fn validate(
		&self,
		// tx_sender: &Hash,
		ledger_state: &LedgerState,
	) -> Result<&Self, OpError>;

	// Applies the operation's changes to the underlying ledger
	fn mut_apply(
		&'a self,
		// tx_sender: &Hash,
		mut_ledger_state: &'a mut LedgerState,
	) -> &'a mut LedgerState;

	// DEFAULTS
	fn is_ledger_mismatched(&self, ledger_state: &LedgerState) -> bool {
		ledger_state.ledger().id().ne(self.ledger_id())
	}
}
