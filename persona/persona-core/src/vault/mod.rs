// mod brain;
mod error;
#[cfg(all(feature = "file"))]
mod file;
mod memory;

pub use error::Error;
pub use memory::Memory;

pub mod prelude {
    use recrypt::api;

    pub use super::{new_derivation_path, Error, VaultClient};
    pub use crate::pubkey::Pubkey;
    pub use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};
    pub use ed25519_dalek_bip32::DerivationPath;
    pub use recrypt::api::{EncryptedValue, TransformBlock, TransformKey};

    pub(crate) const RECRYPT_BIP32_NAME: &str = "recrypt seed";

    pub(crate) type Recrypt =
        api::Recrypt<api::Sha256, api::Ed25519, api::RandomBytes<api::DefaultRng>>;

    ///
    pub type EncryptionKeypair = (PrivateEncryptionKey, PublicEncryptionKey);
    ///
    pub type PrivateEncryptionKey = api::PrivateKey;
    ///
    pub type PublicEncryptionKey = api::PublicKey;
}

use prelude::*;

/// Low-level client access to a vault host (like a ledger, secure enclave, or a file), which itself controls the device's main seed and derived keys.
pub trait VaultClient {
    /// Options for establishing the connection to the vault host.
    type ConnectOpts;

    /// Connects to a vault host, unlocking the device key for further crypto ops.
    fn connect(&mut self, opts: Self::ConnectOpts) -> Result<(), Error>;

    /// The current derivation path used to derive the vault's keypairs.
    fn derivation_path(&self) -> &DerivationPath;

    /// The current public signing key.
    fn public_signing_key(&self) -> Pubkey;

    /// The current public encryption key.
    fn public_encryption_key(&self) -> PublicEncryptionKey;

    /// Signs a message using the currently derived signing keypair.
    fn sign(&mut self, message: &[u8]) -> Result<Signature, Error>;

    /// Verifies a message using the currently derived public signing key.
    fn verify(&self, message: &[u8], signature: &Signature) -> Result<(), Error> {
        PublicKey::from_bytes(self.public_signing_key().as_ref())
            .and_then(|pk| pk.verify_strict(message, signature))
            .map_err(|_| Error::InvalidSignature)
    }

    /// Encrypts a plaintext to the target public encryption key.
    fn encrypt(
        &mut self,
        plaintext: [u8; 384],
        public_encryption_key: &PublicEncryptionKey,
    ) -> Result<EncryptedValue, Error>;

    /// Decrypts a ciphertext intended
    fn decrypt(&mut self, ciphertext: EncryptedValue) -> Result<[u8; 384], Error>;

    // fn reencrypt(&mut self, encrypted: &[u8]) -> Result<>

    // fn issue_jwt(&mut self)
    // fn validate_jwt(&mut self)

    /**
     * TODO: below is wrong, we want a sort of dag-like state machine to handle:
     *  - (decrypting from) multiple hops
     *  -
     */

    ///
    fn generate_transform_key(&mut self, target: &Pubkey) -> Result<TransformKey, Error>;

    ///
    fn transform(
        &mut self,
        encrypted_val: &[u8],
        transform_key: &TransformKey,
    ) -> Result<TransformBlock, Error>;
}

///
pub fn new_derivation_path(address: u32) -> DerivationPath {
    const COIN: u32 = 501;
    const ACCOUNT: u32 = 3282;
    const CHANGE: u32 = 5683;

    DerivationPath::bip44(COIN, ACCOUNT, CHANGE, address).expect("this should not fail")
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek as ed;
    use rand::thread_rng;
    use recrypt::{api::PrivateKey, prelude::*};

    // #[test]
    // fn recrypt_compat() {
    //     let mut rng = thread_rng();
    //     let recrypt = Recrypt::default();

    //     let ed_kp = ed::Keypair::generate(&mut rng);
    //     // let (sk, pk) = recrypt.generate_key_pair().unwrap();
    //     let recrypt_sk = PrivateKey::new_from_slice(ed_kp.secret.as_bytes()).unwrap();

    //     assert_eq!(ed_kp.secret.as_bytes(), recrypt_sk.bytes());

    //     let recrypt_pk = recrypt.compute_public_key(&recrypt_sk).unwrap();
    //     // assert_eq!(ed_kp.public.as_bytes(), recrypt_pk.)
    // }
}
