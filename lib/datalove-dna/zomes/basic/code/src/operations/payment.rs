use std::rc::Rc;
use types::*;
use super::base::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentOperation { // vostro only, unless in HTL
	ledger_id: Hash,
	sender: Hash,
	amount: u128,
	max_amount: Option<u128>,
}

impl PaymentOperation {}

impl<'a> Operation<'a, PaymentError> for PaymentOperation {
	fn ledger_id(&self) -> &Hash { &self.ledger_id }

	fn validate(
		&self,
		ledger_state: &LedgerState,
	) -> Result<&Self, PaymentError> {
		match () {
			_ if false =>
				Err(PaymentError::LedgerIdMismatch),
			_ =>
				Ok(self),
		}
	}

	fn mut_apply(
		&'a self,
		mut_ledger_state: &'a mut LedgerState,
	) -> &'a mut LedgerState {
		mut_ledger_state.mut_ledger().set_balance(self.amount);
		mut_ledger_state
	}
}

quick_error! {
	#[derive(Debug)]
	pub enum PaymentError {
		LedgerIdMismatch {
			description("Operation is intended for another ledger")
		}
	}
}
