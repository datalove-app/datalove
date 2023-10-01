// use chacha20poly1305::
use pwbox::{pure, Eraser, RestoredPwBox};
use toml::from_str;

static KEY_FILE_LOCATION: &str = "~/.datalove/keyfile.toml";

pub enum File {

}

impl File {
    pub fn new() -> Self {
        Self
    }

    fn eraser() -> Eraser {
        let mut eraser = Eraser::new();
        eraser.add_suite::<pure::PureCrypto>();
        eraser
    }
}

impl<C: Core> Service<C> for File {
    type Config = ();

    fn start(_config: Self::Config) -> Self {
        Self::new()
    }
}

impl<C: Core> Driver<C> for File {
    const TYPE: DriverType = DriverType::Keystore;

    type Event = ();
    type Effect = ();

    fn handle_event(&mut self, event: Self::Event) {
        unimplemented!()
    }

    fn apply_effect(&mut self, effect: Self::Effect) {
        unimplemented!()
    }
}

impl<C: Core> Keystore<C> for File {
    type KeyParams = ();

    ///
    fn keypair(&self, params: Self::KeyParams) -> ed25519::Keypair {
        unimplemented!()
    }
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "File")
    }
}
