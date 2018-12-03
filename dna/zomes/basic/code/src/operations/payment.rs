use std::rc::Rc;
use crate::types::*;
use super::base::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentOperation { // vostro only, unless in HTL
    ledger_id: Hash,
    sender: Hash,
    amount: u128,
    max_amount: Option<u128>,
}

impl PaymentOperation {}

impl<'a> Operation<'a, Error> for PaymentOperation {
    fn ledger_id(&self) -> &Hash { &self.ledger_id }

    fn validate(
        &self,
        ledger_history: &LedgerHistory,
    ) -> Result<&Self, Error> {
        match () {
            _ if false =>
                Err(Error::LedgerIdMismatch),
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
        LedgerIdMismatch {
            description("Operation is intended for another ledger")
        }
    }
}
