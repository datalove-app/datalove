use quick_error::quick_error;
use serde_derive::{Serialize, Deserialize};
use super::base::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct IncreaseLimitOperation<'a> { // vostro only, unless in HTL
    #[serde(borrow)]
    ledger_id: OperationLedgerId<'a>,
    amount: u128,
}

impl<'a> IncreaseLimitOperation<'a> {

}

impl<'a> Operation<'a, Error> for IncreaseLimitOperation<'a> {
    fn ledger_id(&self) -> OperationLedgerId<'a> { &self.ledger_id }

    fn validate(
        &self,
        _ledger_history: &LedgerHistory,
    ) -> Result<&Self, Error> {
        match () {
            _ if false =>
                Err(Error::InvalidLimit),
            _ =>
                Ok(self),
        }
    }

    fn mut_apply(
        &'a self,
        mut_ledger_history: &'a mut LedgerHistory,
    ) -> &'a mut LedgerHistory {
        mut_ledger_history.mut_ledger().set_limit(self.amount);
        mut_ledger_history
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        InvalidLimit {
            description("Limit would rise above max u128")
        }
    }
}
