use crate::types::*;
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
            description("Limit would fall below current balance")
        }
    }
}
