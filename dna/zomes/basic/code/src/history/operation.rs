/*  ## Tree based algo:
	on basic:
		map ledger tree
			bump seq_no
			apply tx
	on htl:
		map ledger tree
			bump seq_no
		traverse ledger tree
			apply tx
			append resulting ledger to each existing ledger as a child
	on htl_end:
		map ledger tree
			bump seq_no
		if htl failure tx:
			remove all ledgers (and subtrees) w/ the htl_id as key
		if htl fulfilled tx:
			traverse tree (in-order?)
			if ledger has htl_id as key
				delete newer, "younger" siblings (and subtrees)

 */

// use id_tree::Tree;
use ledger::*;
use operations::*;
use operations::base::*;

pub type LedgerStates = Vec<LedgerState>;

pub struct OperationHistory {
	ledger_states: LedgerStates,
}

impl OperationHistory {
	pub fn new(ledger: Ledger) -> Self {
		let mut ledger_states = Vec::new();
		ledger_states.push(LedgerState::new(ledger));
		OperationHistory { ledger_states: ledger_states }
	}

	pub fn validate(
		&self,
		operation: &LedgerOperation,
	) -> Result<&Self, LedgerOperationError> {
		let op_is_valid = self.ledger_states
			.iter()
			.fold(Ok(()), |are_ledger_states_valid, ledger_state| {
				are_ledger_states_valid
					.and(operation.validate(ledger_state))
					.and(Ok(()))
			});

		match op_is_valid {
			Ok(()) => Ok(&self),
			Err(err) => Err(err),
		}
	}

	pub fn mut_apply(
		&mut self,
		operation: &LedgerOperation,
	) -> &mut Self {
		self.ledger_states
			.iter_mut()
			.for_each(|mut_ls| { operation.mut_apply(mut_ls); });

		self
	}
}
