use types::*;
use super::base::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct IncreaseLimitOperation { // vostro only, unless in HTL
	ledger_id: Hash,
	amount: u128,
}

impl IncreaseLimitOperation {

}

impl<'a> Operation<'a, IncreaseLimitError> for IncreaseLimitOperation {
	fn ledger_id(&self) -> &Hash { &self.ledger_id }

	fn validate(
		&self,
		ledger_state: &LedgerState,
	) -> Result<&Self, IncreaseLimitError> {
		match () {
			_ if false =>
				Err(IncreaseLimitError::LedgerIdMismatch),
			_ =>
				Ok(self),
		}
	}

	fn mut_apply(
		&'a self,
		mut_ledger_state: &'a mut LedgerState,
	) -> &'a mut LedgerState {
		mut_ledger_state.mut_ledger().set_limit(self.amount);
		mut_ledger_state
	}
}

quick_error! {
	#[derive(Debug)]
	pub enum IncreaseLimitError {
		LedgerIdMismatch {
			description("Operation is intended for another ledger")
		}
	}
}
