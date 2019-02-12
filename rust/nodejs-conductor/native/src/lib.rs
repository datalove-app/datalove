#[macro_use] extern crate lazy_static;
#[macro_use] extern crate neon;
extern crate holochain_conductor_api;
extern crate holochain_node_test_waiter;
extern crate neon_serde;

pub mod conductor;

use crate::conductor::JsConductor;

register_module!(mut m, {
    m.export_class::<JsConductor>("Conductor")?;
    Ok(())
});
