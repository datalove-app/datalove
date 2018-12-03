use types::*;
use self::base::*;
use self::set_exchange_rate::*;
use self::increase_limit::*;
use self::decrease_limit::*;
use self::payment::*;
use self::LedgerOperationError as Error;

pub mod base;
mod set_exchange_rate;
mod increase_limit;
mod decrease_limit;
mod payment;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum LedgerOperation {
	SetExchangeRate(SetExchangeRateOperation),
	IncreaseLimit(IncreaseLimitOperation),
	DecreaseLimit(DecreaseLimitOperation),
	Payment(PaymentOperation),
}

impl<'a> Operation<'a, Error> for LedgerOperation {
	fn ledger_id(&self) -> &Hash {
		match self {
			LedgerOperation::SetExchangeRate(op) => op.ledger_id(),
			LedgerOperation::IncreaseLimit(op) => op.ledger_id(),
			LedgerOperation::DecreaseLimit(op) => op.ledger_id(),
			LedgerOperation::Payment(op) => op.ledger_id(),
		}
	}

	fn validate(
		&self,
		ledger_state: &LedgerState,
	) -> Result<&Self, Error> {
		if self.is_ledger_mismatched(ledger_state) {
			return Err(Error::LedgerIdMismatch);
		}

		match self {
			LedgerOperation::SetExchangeRate(op) => op
				.validate(ledger_state)
				.map(|_| self)
				.map_err(Error::SetExchangeRateError),
			LedgerOperation::IncreaseLimit(op) => op
				.validate(ledger_state)
				.map(|_| self)
				.map_err(Error::IncreaseLimitError),
			LedgerOperation::DecreaseLimit(op) => op
				.validate(ledger_state)
				.map(|_| self)
				.map_err(Error::DecreaseLimitError),
			LedgerOperation::Payment(op) => op
				.validate(ledger_state)
				.map(|_| self)
				.map_err(Error::PaymentError),
		}
	}

	fn mut_apply(
		&'a self,
		mut_ledger_state: &'a mut LedgerState,
	) -> &'a mut LedgerState {
		match self {
			LedgerOperation::SetExchangeRate(op) => op
				.mut_apply(mut_ledger_state),
			LedgerOperation::IncreaseLimit(op) => op
				.mut_apply(mut_ledger_state),
			LedgerOperation::DecreaseLimit(op) => op
				.mut_apply(mut_ledger_state),
			LedgerOperation::Payment(op) => op
				.mut_apply(mut_ledger_state),
		};

		mut_ledger_state
			.mut_ledger()
			.bump_seq_no(None);
		mut_ledger_state
	}
}

quick_error! {
	#[derive(Debug)]
	pub enum LedgerOperationError {
		LedgerIdMismatch {
			description("Operation intended for another ledger")
		}
		SetExchangeRateError(err: SetExchangeRateError) {
			description(err.description())
		}
		IncreaseLimitError(err: IncreaseLimitError) {
			description(err.description())
		}
		DecreaseLimitError(err: DecreaseLimitError) {
			description(err.description())
		}
		PaymentError(err: PaymentError) {
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
