use super::prelude::*;
use ed25519_dalek_bip32::ExtendedSecretKey;
use hmac::{Hmac, Mac};
use rand::{thread_rng, Rng};
use recrypt::api::{CryptoOps, KeyGenOps, Plaintext, SigningKeypair};
use sha2::Sha512;
// use secrecy::{ExposeSecret, Secret, SecretVec};
use secstr::{SecBox, SecVec};
use std::str::FromStr;
use zeroize::Zeroize;

type HmacSha512 = Hmac<Sha512>;

///
pub struct Memory {
    pub(crate) seed: SecBox<[u8; 32]>,
    pub(crate) path: DerivationPath,
    pub(crate) current_signing_keypair: Keypair,
    pub(crate) current_encryption_keypair: EncryptionKeypair,
    _recrypt: Recrypt,
}

impl Memory {
    ///
    pub fn random() -> Result<Self, Error> {
        let mut raw_seed = [0u8; 32];
        thread_rng().fill(&mut raw_seed);

        let path = new_derivation_path(1);
        Self::from_seed_and_path(raw_seed, path)
    }

    /// Deserializes the persistable parts from bytes.
    pub(crate) fn from_bytes(bytes: &mut [u8]) -> Result<Self, Error> {
        let mut seed = [0u8; 32];
        seed.copy_from_slice(&bytes[..32]);

        let path_str = std::str::from_utf8(&bytes[32..]).map_err(|_| Error::MalformedData)?;
        let path = DerivationPath::from_str(path_str).map_err(|_| Error::MalformedData)?;

        // zeroize
        bytes.zeroize();

        Self::from_seed_and_path(seed, path)
    }

    /// Serializes the persistable parts to bytes.
    pub(crate) fn to_bytes(&self) -> SecVec<u8> {
        let mut bytes = Vec::new();
        bytes[..32].copy_from_slice(&self.seed.unsecure()[..32]);
        bytes[32..].copy_from_slice(self.path.to_string().as_bytes());
        SecVec::new(bytes)
    }

    ///
    pub fn from_seed_and_path(mut raw_seed: [u8; 32], path: DerivationPath) -> Result<Self, Error> {
        let seed = SecBox::new(Box::new(raw_seed));
        let _recrypt = Recrypt::new();

        let current_signing_keypair = Self::derive_signing_keypair(seed.unsecure(), &path)?;
        let current_encryption_keypair =
            Self::derive_encrpytion_keypair(seed.unsecure(), &path, &_recrypt)?;

        // zeroize
        raw_seed.zeroize();

        Ok(Self {
            seed,
            path,
            current_signing_keypair,
            current_encryption_keypair,
            _recrypt,
        })
    }

    ///
    pub fn derive_signing_keypair(seed: &[u8], path: &DerivationPath) -> Result<Keypair, Error> {
        let extended_sk = ExtendedSecretKey::from_seed(seed)
            .and_then(|esk| esk.derive(path))
            .map_err(|_| Error::MalformedData)?;
        let public = extended_sk.public_key();

        Ok(Keypair {
            secret: extended_sk.secret_key,
            public,
        })
    }

    ///
    pub fn derive_encrpytion_keypair(
        seed: &[u8],
        path: &DerivationPath,
        recrypt: &Recrypt,
    ) -> Result<EncryptionKeypair, Error> {
        /* recreated from ed25519-dalek-bip32 */

        // generate root sk bytes
        let mut mac = HmacSha512::new_from_slice(RECRYPT_BIP32_NAME.as_ref()).unwrap();
        mac.update(seed);
        let mut bytes = mac.finalize().into_bytes();

        let mut sk_bytes = [0u8; 32];
        sk_bytes.copy_from_slice(&bytes[..32]);
        let mut chain_code = [0; 32];
        chain_code.copy_from_slice(&bytes[32..]);

        // derive child for each index in derivation path
        for index in path.as_ref().iter() {
            if index.is_normal() {
                return Err(Error::InvalidDerivationPath);
            }

            mac = HmacSha512::new_from_slice(&chain_code).unwrap();
            mac.update(&[0u8]);
            mac.update(&sk_bytes);
            mac.update(index.to_bits().to_be_bytes().as_ref());
            bytes = mac.finalize().into_bytes();

            sk_bytes.copy_from_slice(&bytes[..32]);
            chain_code.copy_from_slice(&bytes[32..]);
        }

        let encryption_sk = PrivateEncryptionKey::new(sk_bytes);
        let encryption_pk = recrypt
            .compute_public_key(&encryption_sk)
            .map_err(|_| Error::MalformedData)?;

        Ok((encryption_sk, encryption_pk))
    }
}

impl VaultClient for Memory {
    type ConnectOpts = ();

    fn connect(&mut self, _opts: Self::ConnectOpts) -> Result<(), Error> {
        *self = Self::random()?;
        Ok(())
    }

    fn derivation_path(&self) -> &DerivationPath {
        &self.path
    }

    fn public_signing_key(&self) -> Pubkey {
        Pubkey::new_from_array(self.current_signing_keypair.public.to_bytes())
    }

    fn public_encryption_key(&self) -> PublicEncryptionKey {
        self.current_encryption_keypair.1
    }

    fn sign(&mut self, message: &[u8]) -> Result<Signature, Error> {
        Ok(self.current_signing_keypair.sign(message))
    }

    fn encrypt(
        &mut self,
        plaintext: [u8; 384],
        public_encryption_key: &PublicEncryptionKey,
    ) -> Result<EncryptedValue, Error> {
        let plaintext = Plaintext::new(plaintext);

        let mut keypair_bytes = self.current_signing_keypair.to_bytes();
        let keypair =
            SigningKeypair::from_bytes(&keypair_bytes).map_err(|_| Error::InvalidSigningKeypair)?;

        keypair_bytes.zeroize();

        self._recrypt
            .encrypt(&plaintext, public_encryption_key, &keypair)
            .map_err(|_| Error::EncryptionError)
    }

    fn decrypt(&mut self, ciphertext: EncryptedValue) -> Result<[u8; 384], Error> {
        let plaintext = self
            ._recrypt
            .decrypt(ciphertext, &self.current_encryption_keypair.0)
            .map_err(|_| Error::DecryptionError)?;
        Ok(*plaintext.bytes())
    }

    /**
     * TODO: below is wrong, we want a sort of dag-like state machine to handle:
     *  - (decrypting from) multiple hops
     *  -
     */

    ///
    fn generate_transform_key(&mut self, target: &Pubkey) -> Result<TransformKey, Error> {
        unimplemented!()
    }

    ///
    fn transform(
        &mut self,
        encrypted_val: &[u8],
        transform_key: &TransformKey,
    ) -> Result<TransformBlock, Error> {
        unimplemented!()
    }
}
