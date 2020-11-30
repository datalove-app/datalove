use chacha20poly1305::ChaCha20Poly1305;
use datalove_core::dev::*;
use keyring::Keyring;
use pwbox::{Eraser, PwBox, RestoredPwBox};
use rand::thread_rng;
use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};
use zeroize::Zeroize;

static DEFAULT_KEY_FILE: &str = "~/.datalove/ed.enc";
static DEFAULT_USERNAME: &str = "default@datalove";

#[derive(Debug)]
pub struct Config {
    pub key_file: PathBuf,
    pub username: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            key_file: DEFAULT_KEY_FILE.into(),
            username: DEFAULT_USERNAME.into(),
        }
    }
}

/// On the most limited platforms, the `File` keystore is implemented as a
/// password-protected (-encrypted?) file, containing a `libp2p` keypair.
pub enum File {
    Uninit(PathBuf),
    New(PwBox<pwbox::pure::Scrypt, ChaCha20Poly1305>),
    Restored(RestoredPwBox),
}

impl File {
    pub fn random(passphrase: &str) -> (Self, Keypair) {
        let mut rng = thread_rng();
        let keypair = ed25519::Keypair::generate();
        let pwbox = PwBox::new(&mut rng, &passphrase, keypair.encode())
            .expect("should be able to create a new PwBox");

        (Self::New(pwbox), Keypair::Ed25519(keypair))
    }

    /// Produces a `File` from an existing key file, stored at `path`.
    ///
    /// # Panics
    ///
    fn from_path<P: AsRef<Path>>(path: P) -> Self {
        let mut file = fs::File::open(path).expect("key file should exist");
        let mut toml_str = String::new();
        file.read_to_string(&mut toml_str)
            .expect("key file should be readable to a string");

        let erased = toml::from_str(&toml_str).expect("key file should be deserializable");
        let pwbox = Self::eraser()
            .restore(&erased)
            .expect("key file should be restorable with the `pure` crypto suite");

        Self::Restored(pwbox)
    }

    /// Encrypts and stores the key file
    ///
    /// # Panics
    ///
    fn to_path<P: AsRef<Path>>(&self, path: P) {
        if let Self::New(pwbox) = self {
            let mut file = fs::File::create(path).expect("key file should be creatable");
            let erased = Self::eraser()
                .erase(&pwbox)
                .expect("should convert PwBox to serializable form");

            let toml_str =
                toml::to_string_pretty(&erased).expect("should be able to serialize PwBox");
            file.write_all(toml_str.as_bytes())
                .expect("key file should be writable");
        }
    }

    fn eraser() -> Eraser {
        let mut eraser = Eraser::new();
        eraser.add_suite::<pwbox::pure::PureCrypto>();
        eraser
    }
}

impl<C: Core> Service<C> for File {
    type Config = Option<Config>;

    fn start(config: Self::Config) -> Self {
        let Config { key_file, .. } = config.unwrap_or_default();

        // read file path contents into memory
        // if passphrase can decrypt the string
        //  - zero out the passphrase
        //  - deserialize the string into the `File`
        // else,

        if key_file.exists() {
            Self::from_path(key_file)
        } else {
            Self::Uninit(key_file)
        }
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
    ///
    fn unlock(&mut self) -> Result<Keypair, KeystoreError> {
        let mut password = Keyring::new("datalove", DEFAULT_USERNAME)
            .get_password()
            .map_err(|e| KeystoreError::UnlockFailure(format!("{}", e)))?;

        // if uninitialized, create a new random keypair and store it
        if let Self::Uninit(key_file) = self {
            let (keystore, keypair) = Self::random(&password);
            keystore.to_path(key_file);
            *self = keystore;
            return Ok(keypair);
        }

        // otherwise, decrypt convert the bytes into a keypair
        let mut bytes = [0u8; 64];
        match self {
            Self::New(pwbox) => pwbox
                .open_into(&mut bytes, &password)
                .map_err(|e| KeystoreError::UnlockFailure(format!("{}", e)))?,
            Self::Restored(pwbox) => pwbox
                .open_into(&mut bytes, &password)
                .map_err(|e| KeystoreError::UnlockFailure(format!("{}", e)))?,
            _ => unreachable!(),
        };

        // Ok(keypair)
        password.zeroize();
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn create() {}

    #[test]
    fn open() {}
}

// #[derive(Debug, Deserialize, Serialize)]
// pub struct FileRoot {
//     root: SimpleRoot,
// }

// impl Root for FileRoot {
//     fn keypair(&self) -> Keypair {
//         Keypair::Ed25519(self.root.keypair().clone())
//     }

//     fn destroy(self) -> Result<(), RootError> {
//         unimplemented!()
//     }
// }

// impl Peer for FileRoot {
//     fn public_key(&self) -> PublicKey {
//         PublicKey::Ed25519(self.root.public_key())
//     }
// }

// impl ToMultiaddrs for FileRoot {
//     type Iter = <SimpleRoot as ToMultiaddrs>::Iter;
//     fn to_multiaddrs(&self) -> Self::Iter {
//         self.root.to_multiaddrs()
//     }
// }
