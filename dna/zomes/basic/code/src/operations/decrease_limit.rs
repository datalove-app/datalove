use types::*;
use super::base::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct DecreaseLimitOperation { // vostro only, unless in HTL
    ledger_id: Hash,
    amount: u128,
}

impl DecreaseLimitOperation {

}

impl<'a> Operation<'a, Error> for DecreaseLimitOperation {
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
        mut_ledger_history.mut_ledger().set_limit(self.amount);
        mut_ledger_history
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        LedgerIdMismatch {
            description("Operation is intended for another ledger")
        }
        InvalidLimit {
            description("Limit would fall below current balance")
        }
    }
}
