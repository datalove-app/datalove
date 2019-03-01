use std::{
    sync::{
        mpsc::{sync_channel, SyncSender},
        Arc, Mutex,
    },
    time::Duration,
};

// use holochain_core::network::entry_with_header::EntryWithHeader;
use holochain_conductor_api::{
    conductor::Conductor as RustConductor,
    config::{load_configuration, Configuration},
};
use holochain_core::{
    action::Action,
    signal::{signal_channel, Signal, SignalReceiver},
}
use holochain_node_test_waiter::waiter::{
    CallBlockingTask,
    ControlMsg,
    MainBackgroundTask,
};
use neon::{context::Context, prelude::*};

lazy_static! {
    pub static ref INVALID_CONFIG_ERROR: String = String::from(
        "Invalid type specified for config, must be object or string"
    );
}

pub struct Conductor {
    conductor: RustConductor,
    sender_tx: Option<SyncSender<SyncSender<ControlMsg>>>,
    is_running: Arc<Mutex<bool>>,
    is_started: bool,
}

declare_types! {
    pub class JsConductor for Conductor {
        init(mut cx) {
            let config_arg: Handle<JsValue> = cx.argument(0)?;

            let config: Configuration =
                if config_arg.is_a::<JsObject>() {
                    neon_serde::from_value(&mut cx, config_arg)?
                } else if config_arg.is_a::<JsString>() {
                    let toml_str: String = neon_serde
                        ::from_value(&mut cx, config_arg)?;
                    load_configuration(&toml_str)
                        .expect("Could not load TOML config")
                } else {
                    panic!(INVALID_CONFIG_ERROR.clone());
                };

            Ok(Conductor {
                conductor: RustConductor::from_config(config),
                sender_tx: None,
                is_running: Arc::new(Mutex::new(false)),
                is_started: false,
            })
        }

        method start(mut cx) {
            let js_callback: Handle<JsFunction> = cx.argument(0)?;
            let mut this = cx.this();

            let guard = cx.lock();
            let tc = &mut *this.borrow_mut(&guard);
            // tc.sender_tx = Some(sender_tx);
            {
                let mut is_running = tc.is_running.lock().unwrap();
                *is_running = true;
            }

            // tc.conductor.load_config_with_signal()

            Ok(cx.undefined().upcast())
        }

        method stop(mut cx) {
            Ok(cx.undefined().upcast())
        }
    }
}
