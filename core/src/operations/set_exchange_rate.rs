use crate::{
    ledger::*,
    types::*,
};
use super::base::*;

const ZERO: u64 = 0;

#[derive(Serialize, Deserialize, Debug)]
pub struct SetExchangeRateOperation {
    ledger_id: Hash,
    n: u64, // counterparty units
    d: u64, // ledger owner units
}

impl SetExchangeRateOperation {
    fn is_rate_malformed(&self) -> bool { self.d.eq(&ZERO) }

    // to sell one of the ledger owner's units, the amount of receiving counterparty units would exceed the limit
    fn is_ledger_capacity_exceeded(&self, ledger: &Ledger) -> bool {
        let limit = ledger.limit();
        let balance = ledger.balance();
        let (n, d) = ledger.exchange_rate();
        // let min_counterparty_per_owner_units = 0;
        true
    }

    // to buy one of the ledger owner's units, the amount of counterparty units to send would exceed the balance
    fn is_ledger_underfunded(&self, ledger: &Ledger) -> bool {
        let limit = ledger.limit();
        // let balance = ledger.balance();
        // let (n, d) = ledger.exchange_rate();
        true
    }
}

impl<'a> Operation<'a, Error> for SetExchangeRateOperation {
    fn ledger_id(&self) -> &Hash { &self.ledger_id }

    fn validate(
        &self,
        ledger_history: &LedgerHistory,
    ) -> Result<&Self, Error> {
        if self.is_rate_malformed() {
            Err(Error::MalformedExchangeRateError)
        } else if self.is_ledger_capacity_exceeded(&ledger_history.ledger()) {
            Err(Error::ExceededLedgerCapacityError)
        } else if self.is_ledger_underfunded(&ledger_history.ledger()) {
            Err(Error::UnderfundedLedgerError)
        } else {
            Ok(self)
        }
    }

    fn mut_apply(
        &'a self,
        mut_ledger_history: &'a mut LedgerHistory,
    ) -> &'a mut LedgerHistory {
        mut_ledger_history.mut_ledger().set_exchange_rate((self.n, self.d));
        mut_ledger_history
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
