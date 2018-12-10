use quick_error::quick_error;
use serde_derive::{Serialize, Deserialize};
use crate::types::*;
use super::base::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentOperation<'a> { // vostro only, unless in HTL
    #[serde(borrow)]
    ledger_id: OperationLedgerId<'a>,
    sender: Hash,

    /// Amount to send, denominated in receiver's ledger units.
    /// If in a basic tx, units also the current ledger's units
    amount: u128,

    /// Maximum amount to send, denominated in current ledger units.
    /// Used as a reference point of the amount not to exceed while calculating
    /// available liquidity at a local exchange rate.
    max_amount: Option<u128>,
}

impl<'a> PaymentOperation<'a> {}

impl<'a> Operation<'a, Error> for PaymentOperation<'a> {
    fn ledger_id(&self) -> OperationLedgerId<'a> { &self.ledger_id }

    fn validate(
        &self,
        _ledger_history: &LedgerHistory,
    ) -> Result<&Self, Error> {
        match () {
            _ if false =>
                Err(Error::InvalidPayment),
            _ =>
                Ok(self),
        }
    }

    fn mut_apply(
        &'a self,
        mut_ledger_history: &'a mut LedgerHistory,
    ) -> &'a mut LedgerHistory {
        mut_ledger_history.mut_ledger().set_balance(self.amount);
        mut_ledger_history
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        InvalidPayment {
            description("Limit would fall below current balance")
        }
    }
}
