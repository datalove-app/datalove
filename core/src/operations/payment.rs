use std::rc::Rc;
use quick_error::quick_error;
use serde_derive::{Serialize, Deserialize};
use crate::ledger::*;
use crate::types::*;
use super::base::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentOperation { // vostro only, unless in HTL
    ledger_id: LedgerId,
    sender: Hash,

    /// Amount to send, denominated in receiver's ledger units.
    /// If in a basic tx, units also the current ledger's units
    amount: u128,

    /// Maximum amount to send, denominated in current ledger units.
    /// Used as a reference point of the amount not to exceed while calculating
    /// available liquidity at a local exchange rate.
    max_amount: Option<u128>,
}

impl PaymentOperation {}

impl<'a> Operation<'a, Error> for PaymentOperation {
    fn ledger_id(&self) -> LedgerId { Rc::clone(&self.ledger_id) }

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
