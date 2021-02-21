//! DEPRECATED, use `file.rs` instead

use datalove_core::dev::*;
use rand::thread_rng;
use std::{
    convert::TryInto,
    fmt::{self, Write},
};
use zeroize::Zeroize;

static SERVICE: &str = "datalove_root";
static USERNAME: &str = "current_user@datalove";

/// To increase ease-of-use on desktop platforms with pre-existing user login
/// systems, the `Keyring` keystore is implemented as a
/// sequence of bytes representing a `libp2p` keypair, stored in the operating
/// system's keyring.
pub struct Keyring(keyring::Keyring<'static>);

impl Keyring {
    #[inline(always)]
    pub fn new() -> Self {
        Self(keyring::Keyring::new(SERVICE, USERNAME))
    }

    fn get_or_create_keypair(&self, force_create: bool) -> Keypair {
        use keyring::KeyringError::*;

        match self.0.get_password() {
            Err(Parse(e)) => panic!("MUSTFIX: must not have parse errors: {}", e),
            Err(SecretServiceError(e)) => panic!("received SecretService error: {}", e),
            Err(NoBackendFound) => {
                panic!("platform does not support keyring or SecretService not found")
            }
            Err(NoPasswordFound) => self.upsert_random_keypair(),
            Ok(_) if force_create => self.upsert_random_keypair(),
            Ok(keystr) => {
                let mut bytes = Self::keystr_to_bytes(keystr);
                let keypair = ed25519::Keypair::decode(&mut bytes)
                    .expect("stored key should be decodable into libp2p Keypair");
                Keypair::Ed25519(keypair)
            }
        }
    }

    fn upsert_random_keypair(&self) -> Keypair {
        let keypair = ed25519::Keypair::generate();
        let mut keystr = Self::bytes_to_keystr(keypair.encode());

        self.0
            .set_password(&keystr)
            .expect("MUSTFIX: must be able to set a keystr in the keyring");
        keystr.zeroize();

        Keypair::Ed25519(keypair)
    }

    fn bytes_to_keystr(mut bytes: [u8; 64]) -> String {
        let mut keystr = String::with_capacity(2 * bytes.len());
        for b in bytes.iter() {
            write!(keystr, "{:02X}", b).expect("should not fail to write bytes as hex to string");
        }
        bytes.zeroize();
        keystr
    }

    fn keystr_to_bytes(mut keystr: String) -> [u8; 64] {
        let mut bytes = [0u8; 64];
        for i in (0..keystr.len()).step_by(2) {
            bytes[i] = u8::from_str_radix(&keystr[i..i + 2], 16)
                .expect("should not fail to parse 2 chars as byte");
        }
        keystr.zeroize();
        bytes
    }
}

impl<C: Core> Service<C> for Keyring {
    type Config = ();

    fn start(_config: Self::Config) -> Self {
        Self::new()
    }
}

impl<C: Core> Driver<C> for Keyring {
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

impl<C: Core> Keystore<C> for Keyring {
    type LockParams = bool;

    ///
    fn unlock(&mut self, lock_params: Self::LockParams) -> Result<Keypair, KeystoreError> {
        Ok(self.get_or_create_keypair(lock_params))
    }
}

impl fmt::Debug for Keyring {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "Keyring {{ service: {}, username: {} }}",
            SERVICE, USERNAME
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        // let mut file = File::new(None);
        // println!("{:?}", &file);
        let password = File::get_password(Some("sunnyg")).unwrap();
        println!("{}", &password);
    }

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
