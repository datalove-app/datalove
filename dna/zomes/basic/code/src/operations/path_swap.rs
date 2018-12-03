
use ledger::*;
use types::*;
use super::base::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct PathSwapOperation { // htl only
    ledger_id: Rc<Hash>,
    sender: Hash,
    swap_ledger_id: Rc<Hash>,
    swap_ledger_counterparty: Hash,
    amount: u128,
    max_amount: u128,
}

impl PathSwapOperation {}

impl<'a> Operation<'a, PathSwapError> for PathSwapOperation {
    fn ledger_id(&self) -> &Hash { &self.ledger_id }

    fn validate(
        &self,
        ledger_history: &'a LedgerState,,
        _history: &LedgerHistory,
    ) -> Result<&Self, PathSwapError> {
        match () {
            _ if false =>
                Err(PathSwapError::LedgerIdMismatch),
            _ =>
                Ok(self),
        }
    }

    fn mut_apply(
        &'a self,
        mut_ledger_history: &'a mut LedgerHistory,
        history: &LedgerHistory,
    ) -> &'a mut Ledger {
        mut_ledger.set_balance(self.amount)
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum PathSwapError {
        LedgerIdMismatch {
            description("Operation is intended for another ledger")
        }
    }
}

impl OperationError for PathSwapError {}
