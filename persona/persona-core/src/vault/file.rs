use super::{prelude::*, Memory};
use pwbox::{pure, Eraser};
use secstr::SecUtf8;
// use secrecy::{ExposeSecret, Secret, SecretString, SecretVec};
use std::{fs, path::PathBuf, str::FromStr};

pub struct File(Memory);

impl File {
    ///
    pub fn try_open(opts: FileOpts) -> Result<Self, Error> {
        let file = fs::read(&opts.path).map_err(Error::io)?;
        let file = String::from_utf8(file).map_err(|_| Error::MalformedData)?;

        let mut seed_and_path = Vec::new();
        let pwbox = toml::from_str(&file).map_err(|_| Error::MalformedData)?;
        Self::eraser()
            .restore(&pwbox)
            .and_then(|bytes| bytes.open_into(&mut seed_and_path, &opts.passwd.unsecure()))
            .map_err(|_| Error::MalformedData)?;

        Ok(Self(Memory::from_bytes(&mut seed_and_path)?))
    }

    ///
    pub fn random() -> Result<Self, Error> {
        Ok(Self(Memory::random()?))
    }

    fn eraser() -> Eraser {
        let mut eraser = Eraser::new();
        eraser.add_suite::<pure::PureCrypto>();
        eraser
    }
}

pub struct FileOpts {
    pub path: PathBuf,
    pub passwd: SecUtf8,
}

impl VaultClient for File {
    type ConnectOpts = FileOpts;

    fn connect(&mut self, opts: Self::ConnectOpts) -> Result<(), Error> {
        let default_path = new_derivation_path(0);
        if self.0.path == default_path {
            *self = Self::try_open(opts)?;
        }

        Ok(())
    }

    fn derivation_path(&self) -> &DerivationPath {
        VaultClient::derivation_path(&self.0)
    }

    fn public_signing_key(&self) -> Pubkey {
        VaultClient::public_signing_key(&self.0)
    }

    fn public_encryption_key(&self) -> PublicEncryptionKey {
        VaultClient::public_encryption_key(&self.0)
    }

    fn sign(&mut self, message: &[u8]) -> Result<Signature, Error> {
        VaultClient::sign(&mut self.0, message)
    }

    fn encrypt(
        &mut self,
        plaintext: [u8; 384],
        public_encryption_key: &PublicEncryptionKey,
    ) -> Result<EncryptedValue, Error> {
        VaultClient::encrypt(&mut self.0, plaintext, public_encryption_key)
    }

    fn decrypt(&mut self, ciphertext: EncryptedValue) -> Result<[u8; 384], Error> {
        VaultClient::decrypt(&mut self.0, ciphertext)
    }

    /**
     * TODO: below is wrong, we want a sort of dag-like state machine to handle:
     *  - (decrypting from) multiple hops
     *  -
     */

    ///
    fn generate_transform_key(&mut self, target: &Pubkey) -> Result<TransformKey, Error> {
        VaultClient::generate_transform_key(&mut self.0, target)
    }

    ///
    fn transform(
        &mut self,
        encrypted_val: &[u8],
        transform_key: &TransformKey,
    ) -> Result<TransformBlock, Error> {
        VaultClient::transform(&mut self.0, encrypted_val, transform_key)
    }
}
