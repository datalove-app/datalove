use std::rc::Rc;
use quick_error::quick_error;
use serde_derive::{Serialize, Deserialize};
use crate::ledger::*;
use super::base::*;

const ZERO: u64 = 0;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SetExchangeRateOperation {
    ledger_id: LedgerId,
    n: u64, // counterparty units
    d: u64, // ledger owner units
}

impl SetExchangeRateOperation {
    fn is_rate_malformed(&self) -> bool { self.d.eq(&ZERO) }

    // to sell one of the ledger owner's units, the amount of receiving counterparty units would exceed the limit
    fn is_ledger_capacity_exceeded(&self, ledger: &Ledger) -> bool {
        let _limit = ledger.limit();
        let _balance = ledger.balance();
        let (_n, _d) = ledger.exchange_rate();
        // let min_counterparty_per_owner_units = 0;
        true
    }

    // to buy one of the ledger owner's units, the amount of counterparty units to send would exceed the balance
    fn is_ledger_underfunded(&self, ledger: &Ledger) -> bool {
        let _limit = ledger.limit();
        // let balance = ledger.balance();
        // let (n, d) = ledger.exchange_rate();
        true
    }
}

impl<'a> Operation<'a, Error> for SetExchangeRateOperation {
    fn ledger_id(&self) -> LedgerId { Rc::clone(&self.ledger_id) }

    fn validate(
        &self,
        context: &OperationContext,
    ) -> Result<&Self, Error> {
        if self.is_rate_malformed() {
            Err(Error::MalformedExchangeRateError)
        } else if self.is_ledger_capacity_exceeded(&context.ledger()) {
            Err(Error::ExceededLedgerCapacityError)
        } else if self.is_ledger_underfunded(&context.ledger()) {
            Err(Error::UnderfundedLedgerError)
        } else {
            Ok(self)
        }
    }

    fn mut_apply(
        &'a self,
        mut_context: &'a mut OperationContext,
    ) -> &'a mut OperationContext {
        mut_context
            .mut_ledger()
            .set_exchange_rate((self.n, self.d));
        mut_context
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        MalformedExchangeRateError {
            description("Malformed exchange rate")
        }
        ExceededLedgerCapacityError {
            description("Exchange rate would require that the ledger receive more than the current limit")
        }
        UnderfundedLedgerError {
            description("Exchange rate would require that the ledger spend more than it's available balance")
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
