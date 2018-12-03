use ledger::*;
use types::*;
use super::base::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct PathPaymentOperation { // htl only
    ledger_id: Rc<Hash>,
    sender: Hash,
    amount: u128,
    max_amount: u128,
}

impl PathPaymentOperation {}

impl<'a> Operation<'a, PathPaymentError> for PathPaymentOperation {
    fn is_htl_only(&self) -> bool { true }

    fn ledger_id(&self) -> &Hash { &self.ledger_id }

    fn validate(
        &self,
        ledger_history: &'a LedgerState,
    ) -> Result<&Self, PathPaymentError> {
        match () {
            _ if false =>
                Err(PathPaymentError::LedgerIdMismatch),
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
    pub enum PathPaymentError {
        LedgerIdMismatch {
            description("Operation is intended for another ledger")
        }
    }
}

impl OperationError for PathPaymentError {}
