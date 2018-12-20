use std::rc::Rc;
use quick_error::quick_error;
use serde_derive::{Serialize, Deserialize};
use crate::{
    ledger::LedgerIdRc,
    types::AgentAddress,
};
use super::base::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentOperation { // vostro only, unless in HTL
    ledger_id: LedgerIdRc,
    payer: AgentAddress,

    /**
     * Amount to send, denominated in receiver's ledger units.
     * If in a basic tx, these are also the current ledger's units.
     */
    amount: u128,

    /**
     * Maximum amount to send, denominated in current ledger units.
     * Used as a reference point of the amount not to exceed while calculating
     * available liquidity at a local exchange rate.
     */
    max_amount: Option<u128>,
}

impl PaymentOperation {}

impl<'a> Operation<'a, Error> for PaymentOperation {
    fn ledger_id(&self) -> LedgerIdRc { Rc::clone(&self.ledger_id) }

    fn validate(
        &self,
        context: &OperationContext,
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
        mut_context: &'a mut OperationContext,
    ) -> &'a mut OperationContext {
        mut_context
            .mut_ledger()
            .set_balance(self.amount);
        mut_context
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
