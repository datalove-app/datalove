use std::rc::Rc;
use quick_error::quick_error;
use serde_derive::{Serialize, Deserialize};
use crate::ledger::LedgerIdRc;
use super::base::{Context, Operation};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IncreaseLimitOperation { // vostro only, unless in HTL
    ledger_id: LedgerIdRc,
    amount: u128,
}

impl IncreaseLimitOperation {

}

impl<'a> Operation<'a> for IncreaseLimitOperation {
    type Error = Error;

    fn ledger_id(&self) -> LedgerIdRc { Rc::clone(&self.ledger_id) }

    fn validate(
        &self,
        context: &Context,
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
        mut_context: &'a mut Context,
    ) -> &'a mut Context {
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
            description("Limit would rise above max u128")
        }
    }
}
