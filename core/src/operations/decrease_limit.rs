use std::rc::Rc;
use quick_error::quick_error;
use serde_derive::{Serialize, Deserialize};
use crate::ledger::LedgerIdRc;
use super::base::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DecreaseLimitOperation { // vostro only, unless in HTL
    ledger_id: LedgerIdRc,
    amount: u128,
}

impl DecreaseLimitOperation {

}

impl<'a> Operation<'a, Error> for DecreaseLimitOperation {
    fn ledger_id(&self) -> LedgerIdRc { Rc::clone(&self.ledger_id) }

    fn validate(
        &self,
        context: &OperationContext,
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
        mut_context: &'a mut OperationContext,
    ) -> &'a mut OperationContext {
        mut_context
            .mut_ledger()
            .set_limit(self.amount);
        mut_context
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        InvalidLimit {
            description("Limit would fall below current balance")
        }
    }
}
