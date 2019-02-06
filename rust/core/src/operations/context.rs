use std::collections::HashMap;
use crate::{
    ledger::Ledger,
    operations::base::{
        Context as IOperationContext,
        Effects as OperationEffects,
    },
};

/**
 * Stores the ledger state and any side effects of applying an operation
 */
pub struct OperationContext {
    sender: Option<&'static str>,
    ledger: Ledger,
    effects: OperationEffects,
}

impl OperationContext {
    pub fn new(ledger: Ledger) -> Self {
        OperationContext {
            sender: None,
            ledger,
            effects: HashMap::new()
        }
    }
}

impl IOperationContext for OperationContext {
    fn sender(&self) -> &str { &self.sender.unwrap() }
    fn ledger(&self) -> &Ledger { &self.ledger }
    fn effects(&self) -> &OperationEffects { &self.effects }

    fn mut_ledger(&mut self) -> &mut Ledger { &mut self.ledger }
    fn mut_effects(&mut self) -> &mut OperationEffects { &mut self.effects }
}
