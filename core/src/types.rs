use std::rc::Rc;
use holochain_core_types::hash::HashString;

pub type AgentAddress = HashString;
pub type AgentAddressRc = Rc<AgentAddress>;
