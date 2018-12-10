use quick_error::quick_error;
use serde_derive::{Serialize, Deserialize};
use self::{
    base::*,
    set_exchange_rate::{Error as SetExchangeRateError, *},
    increase_limit::{Error as IncreaseLimitError, *},
    decrease_limit::{Error as DecreaseLimitError, *},
    payment::{Error as PaymentError, *},
};

pub mod base;
pub mod set_exchange_rate;
pub mod increase_limit;
pub mod decrease_limit;
pub mod payment;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum LedgerOperation<'a> {
    #[serde(borrow)]
    SetExchangeRate(SetExchangeRateOperation<'a>),
    #[serde(borrow)]
    IncreaseLimit(IncreaseLimitOperation<'a>),
    #[serde(borrow)]
    DecreaseLimit(DecreaseLimitOperation<'a>),
    #[serde(borrow)]
    Payment(PaymentOperation<'a>),
}

impl<'a> Operation<'a, Error> for LedgerOperation<'a> {
    fn ledger_id(&self) -> OperationLedgerId<'a> {
        match self {
            LedgerOperation::SetExchangeRate(op) => op.ledger_id(),
            LedgerOperation::IncreaseLimit(op) => op.ledger_id(),
            LedgerOperation::DecreaseLimit(op) => op.ledger_id(),
            LedgerOperation::Payment(op) => op.ledger_id(),
        }
    }

    fn validate(
        &self,
        ledger_history: &LedgerHistory,
    ) -> Result<&Self, Error> {
        if self.is_ledger_mismatched(ledger_history) {
            return Err(Error::LedgerIdMismatch);
        }

        match self {
            LedgerOperation::SetExchangeRate(op) => op
                .validate(ledger_history)
                .and(Ok(self))
                .map_err(Error::SetExchangeRateError),
            LedgerOperation::IncreaseLimit(op) => op
                .validate(ledger_history)
                .and(Ok(self))
                .map_err(Error::IncreaseLimitError),
            LedgerOperation::DecreaseLimit(op) => op
                .validate(ledger_history)
                .and(Ok(self))
                .map_err(Error::DecreaseLimitError),
            LedgerOperation::Payment(op) => op
                .validate(ledger_history)
                .and(Ok(self))
                .map_err(Error::PaymentError),
        }
    }

    fn mut_apply(
        &'a self,
        mut_ledger_history: &'a mut LedgerHistory,
    ) -> &'a mut LedgerHistory {
        match self {
            LedgerOperation::SetExchangeRate(op) => op
                .mut_apply(mut_ledger_history),
            LedgerOperation::IncreaseLimit(op) => op
                .mut_apply(mut_ledger_history),
            LedgerOperation::DecreaseLimit(op) => op
                .mut_apply(mut_ledger_history),
            LedgerOperation::Payment(op) => op
                .mut_apply(mut_ledger_history),
        }
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
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
