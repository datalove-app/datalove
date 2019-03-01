pub mod conductor;

use crate::conductor::JsConductor;

register_module!(mut m, {
    m.export_class::<JsConductor>("Conductor")?;
    Ok(())
});
