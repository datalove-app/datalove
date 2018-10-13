use types::*;
use super::base::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct DecreaseLimitOperation { // vostro only, unless in HTL
	ledger_id: Hash,
	amount: u128,
}

impl DecreaseLimitOperation {

}

impl<'a> Operation<'a, DecreaseLimitError> for DecreaseLimitOperation {
	fn ledger_id(&self) -> &Hash { &self.ledger_id }

	fn validate(
		&self,
		ledger_state: &LedgerState,
	) -> Result<&Self, DecreaseLimitError> {
		match () {
			_ if false =>
				Err(DecreaseLimitError::LedgerIdMismatch),
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
	pub enum DecreaseLimitError {
		LedgerIdMismatch {
			description("Operation is intended for another ledger")
		}
		InvalidLimit {
			description("Limit would fall below current balance")
		}
	}
}
