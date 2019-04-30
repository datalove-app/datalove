use std::rc::Rc;
use holochain_core_types::{
    error::HolochainError,
    hash::HashString,
    json::JsonString,
};
use holochain_core_types_derive::DefaultJson;
use serde_derive::{Serialize, Deserialize};
use crate::types::{AgentAddress, AgentAddressRc};

pub type LedgerId = HashString;
pub type LedgerIdRc = Rc<LedgerId>;
pub type LedgerMetadata = HashString;
pub type LedgerMetadataRc = Rc<LedgerMetadata>;
pub type LedgerExchangeRate = (u64, u64);

pub const ENTRY_TYPE_NAME: &'static str = "ledger";

/**
 * The main struct to which all operations and transactions are applied.
 *
 * The core purpose of a `Ledger` is to track the evolving existance and
 * ownership of an abstract, singly-created and mutually-agreed-upon number,
 * which can represent a quantity of anything that can be owned and exchanged.
 */
#[derive(Serialize, Deserialize, DefaultJson, Clone, Debug)]
pub struct Ledger {
    // configuration state
    min_timeout: u32, // TODO: could these be in seq_no units?
    max_timeout: u32,
    max_pending_htls: u8,
    max_ops_per_transaction: u8,

    // ledger (app) state
    id: LedgerIdRc,
    issuer: AgentAddressRc,
    owner: AgentAddressRc,
    limit: u128,
    balance: u128,
    exchange_rate: LedgerExchangeRate,
    metadata: LedgerMetadataRc,
    // latest_tx_entry_count

    // ledger (history) state
    seq_no: u64, // TODO: is this necessary?
    // TODO: can the strings be Rc<String>?
    latest_tx_entry_hash: Option<Rc<HashString>>, // TODO: is this necessary?
}

// TODO: needs logic to:
/*  dictate how basic transactions and operations are applied and can be synchronized/caught up
 */
/*  dictate how a (basic or HTL) transaction can be applied to a ledger with pending HTL(s)
    - updates to general fields:
        - updates made by one party with static invariants
        - e.g. exchange_rate
    - updates to limit:
        - updates made by one party with static (e.g. increase must be less than max_u128) and field (e.g. decrease can't be below balance) invariants
    - updates to seq_no:
        - updates made by either party with static invariants (must be one greater than seq_no on current ledger)
    - updates to balance:
        - updates made by either party with static and field invariants
 */
/*	dictate if and how an HTL (nested behind other applied transactions) is failed/fulfilled
    - any new failed/fulfilled htl tx:
        - update original tx as marked as failed/fulfilled
        - if this fails/fulfills the first pending htl in the list (i.e. the oldest):
            - remove it from the list, apply it to ledger(s)
            - for every subsequent txn, until the next pending htl:
                - if basic: apply it to ledger(s)
                - if htl and failed/fulfilled: remove from list, apply it to ledger
            - at this point, the decided history of confirmed txns should be already applied to the ledger(s), so commit it/them
        - else (if it fails/fulfills an htl in the middle of the list):
            - remove it from list
 */
impl Ledger {
    // GETTERS
    pub fn id(&self) -> LedgerIdRc { Rc::clone(&self.id) }
    pub fn owner(&self) -> AgentAddressRc { Rc::clone(&self.owner) }
    pub fn issuer(&self) -> AgentAddressRc { Rc::clone(&self.issuer) }

    pub fn seq_no(&self) -> u64 { self.seq_no }
    pub fn limit(&self) -> u128 { self.limit }
    pub fn balance(&self) -> u128 { self.balance }
    pub fn exchange_rate(&self) -> &LedgerExchangeRate { &self.exchange_rate }
    pub fn metadata(&self) -> LedgerMetadataRc { Rc::clone(&self.metadata) }

    // SETTERS
    pub fn bump_seq_no(&mut self, new_seq_no: Option<u64>) -> u64 {
        match new_seq_no {
            None => self.seq_no += 1,
            Some(seq_no) => self.seq_no = seq_no,
        }
        self.seq_no
    }
    pub fn set_limit(&mut self, limit: u128) -> &mut Self {
        self.limit = limit;
        self
    }
    pub fn set_balance(&mut self, balance: u128) -> &mut Self {
        self.balance = balance;
        self
    }
    pub fn set_exchange_rate(
        &mut self,
        exchange_rate: LedgerExchangeRate
    ) -> &mut Self {
        self.exchange_rate = exchange_rate;
        self
    }

    // pub fn set_metadata(&self) -> &str { &self.metadata }

    // fn from_json(json: serde_json::Value) -> Result<Ledger, serde_json::Error> {
    //     serde_json::from_value(json)
    // }

    // fn to_json(&self) -> Result<serde_json::Value, serde_json::Error> {
    //     serde_json::to_value(self)
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(true);
    }
}

