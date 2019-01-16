use quick_error::quick_error;
use serde_derive::{Serialize, Deserialize};
use crate::ledger::*;
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

/**
 *
 */
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum LedgerOperation {
    SetExchangeRate(SetExchangeRateOperation),
    IncreaseLimit(IncreaseLimitOperation),
    DecreaseLimit(DecreaseLimitOperation),
    Payment(PaymentOperation),
}

impl<'a> LedgerOperation {
    pub fn ledger_id(&self) -> LedgerIdRc {
        match self {
            LedgerOperation::SetExchangeRate(op) => op.ledger_id(),
            LedgerOperation::IncreaseLimit(op) => op.ledger_id(),
            LedgerOperation::DecreaseLimit(op) => op.ledger_id(),
            LedgerOperation::Payment(op) => op.ledger_id(),
        }
    }

    pub fn validate(
        &self,
        context: &OperationContext,
    ) -> Result<&Self, Error> {
        self.verify_ledger_id_match(context)?;

        match self {
            LedgerOperation::SetExchangeRate(op) => op
                .validate(context)
                .and(Ok(self))
                .map_err(Error::SetExchangeRateError),
            LedgerOperation::IncreaseLimit(op) => op
                .validate(context)
                .and(Ok(self))
                .map_err(Error::IncreaseLimitError),
            LedgerOperation::DecreaseLimit(op) => op
                .validate(context)
                .and(Ok(self))
                .map_err(Error::DecreaseLimitError),
            LedgerOperation::Payment(op) => op
                .validate(context)
                .and(Ok(self))
                .map_err(Error::PaymentError),
        }
    }

    pub fn mut_apply(
        &'a self,
        context: &'a mut OperationContext,
    ) -> &'a mut OperationContext {
        match self {
            LedgerOperation::SetExchangeRate(op) => op.mut_apply(context),
            LedgerOperation::IncreaseLimit(op) => op.mut_apply(context),
            LedgerOperation::DecreaseLimit(op) => op.mut_apply(context),
            LedgerOperation::Payment(op) => op.mut_apply(context),
        }
    }

    /**
     * Determines if operation is destined for this ledger, or for another.
     */
    fn verify_ledger_id_match(
        &self,
        context: &OperationContext,
    ) -> Result<&Self, Error> {
        if context.ledger().id().eq(&self.ledger_id()) {
            Ok(self)
        } else {
            Err(Error::LedgerIdMismatch)
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
