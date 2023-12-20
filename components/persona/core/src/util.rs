pub use self::digest::DigestPipe;
pub use risc0::{ImageId, Sha256Digest, Sha256Pipe};

use crate::maybestd::{
    cell::{self, Ref, RefCell},
    collections::VecDeque,
    fmt, io,
    ops::{BitXor, BitXorAssign},
    str::FromStr,
    vec::Vec,
};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug)]
pub struct Empty;
impl io::Read for Empty {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        Ok(0)
    }
}
impl io::Write for Empty {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub mod digest {
    use super::*;
    use ::digest::Digest;
    pub struct DigestPipe<D, T>(D, T);

    impl<D: Digest, R: io::Read> DigestPipe<D, R> {
        pub fn decode_from_reader<T: BorshDeserialize>(reader: R) -> io::Result<(D, T)> {
            let mut this = Self::from(reader);
            let outputs = T::deserialize_reader(&mut this)?;
            Ok((this.0, outputs))
        }
    }

    impl<D: Digest, W: io::Write> DigestPipe<D, W> {
        pub fn encode_to_writer<T: BorshSerialize>(val: &T, writer: W) -> io::Result<D> {
            let mut this = Self::from(writer);
            val.serialize(&mut this)?;
            Ok(this.0)
        }
    }

    impl<D: Digest, T> From<T> for DigestPipe<D, T> {
        fn from(pipe: T) -> Self {
            Self(D::new(), pipe)
        }
    }

    impl<D: Digest, R: io::Read> io::Read for DigestPipe<D, R> {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let read = self.1.read(buf)?;
            self.0.update(&buf[..read]);
            Ok(read)
        }
    }

    impl<D: Digest, W: io::Write> io::Write for DigestPipe<D, W> {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            let write = self.1.write(buf)?;
            self.0.update(&buf[..write]);
            Ok(write)
        }

        fn flush(&mut self) -> io::Result<()> {
            self.1.flush()
        }
    }
}

pub mod ed25519 {
    use super::*;
    use ed25519_dalek::{Signature, VerifyingKey};

    pub fn serialize_key<W: io::Write>(vk: &VerifyingKey, writer: &mut W) -> io::Result<()> {
        vk.to_bytes().serialize(writer)
    }

    pub fn deserialize_key<R: io::Read>(reader: &mut R) -> io::Result<VerifyingKey> {
        let vk_bytes = <[u8; 32]>::deserialize_reader(reader)?;
        VerifyingKey::from_bytes(&vk_bytes).map_err(|_| io::ErrorKind::InvalidData.into())
    }

    pub fn serialize_signature<W: io::Write>(
        signature: &Signature,
        writer: &mut W,
    ) -> io::Result<()> {
        signature.to_bytes().serialize(writer)
    }

    pub fn deserialize_signature<R: io::Read>(reader: &mut R) -> io::Result<Signature> {
        <[u8; 64]>::deserialize_reader(reader).map(|bytes| Signature::from_bytes(&bytes))
    }

    pub fn serialize_signatures<W: io::Write, T: AsRef<[Signature]>>(
        signatures: T,
        writer: &mut W,
    ) -> io::Result<()> {
        let signatures = signatures.as_ref();
        let len = u32::try_from(signatures.len()).map_err(|_| io::ErrorKind::InvalidInput)?;
        writer.write_all(&len.to_le_bytes())?;
        for signature in signatures {
            serialize_signature(signature, writer)?;
        }
        Ok(())
    }

    pub fn deserialize_signatures<R: io::Read>(reader: &mut R) -> io::Result<Vec<Signature>> {
        let len = <[u8; 4]>::deserialize_reader(reader).map(u32::from_le_bytes)?;
        (0..len)
            .map(|_| deserialize_signature(reader))
            .collect::<io::Result<Vec<_>>>()
    }
}

pub mod risc0 {
    use super::*;
    use ::digest::Digest;
    use hex::FromHex;
    use risc0_zkvm::{
        sha::{self, DIGEST_BYTES, DIGEST_WORDS},
        ReceiptClaim,
    };

    pub use sha::rust_crypto::Sha256;

    pub type ImageId = Sha256Digest;

    pub fn trace(fmt: fmt::Arguments, prev: Option<usize>) -> usize {
        #[cfg(target_os = "zkvm")]
        {
            let cc = risc0_zkvm::guest::env::get_cycle_count();
            eprintln!("<guest> [{}] {}", cc - prev.unwrap_or_default(), fmt);
            cc
        }
        #[cfg(not(target_os = "zkvm"))]
        {
            eprintln!("{}", fmt);
            0
        }
    }

    #[cfg(target_os = "zkvm")]
    pub fn self_image_id() -> ImageId {
        // static mut IMAGE_ID: cell::OnceCell<Sha256Digest> = cell::OnceCell::new();

        // IMAGE_ID.get_or_init(|| {
        let mut hex_words = [0u32; 16];
        let len = risc0_zkvm::guest::env::env_var("SELF_IMAGE_ID", &mut hex_words);
        let image_id = hex::FromHex::from_hex(bytemuck::cast_slice(&hex_words))
            .expect("invalid image_id hex words");
        // })
        eprintln!("self_image_id: {:?}", &image_id);
        image_id
    }

    ///
    #[derive(Copy, Clone, Debug, Default, Eq, PartialEq, BorshDeserialize, BorshSerialize)]
    #[repr(transparent)]
    pub struct Sha256Digest(
        #[borsh(
            deserialize_with = "deserialize_digest",
            serialize_with = "serialize_digest"
        )]
        sha::Digest,
    );

    impl Sha256Digest {
        pub const ZERO: Self = Self(sha::Digest::ZERO);
    }

    impl<T: ?Sized> AsRef<T> for Sha256Digest
    where
        sha::Digest: AsRef<T>,
    {
        #[inline]
        fn as_ref(&self) -> &T {
            self.0.as_ref()
        }
    }

    impl From<[u8; DIGEST_BYTES]> for Sha256Digest {
        #[inline]
        fn from(digest: [u8; DIGEST_BYTES]) -> Self {
            Self(sha::Digest::from(digest))
        }
    }

    impl From<[u32; DIGEST_WORDS]> for Sha256Digest {
        #[inline]
        fn from(digest: [u32; DIGEST_WORDS]) -> Self {
            Self(sha::Digest::from(digest))
        }
    }

    impl From<&Sha256> for Sha256Digest {
        #[inline]
        fn from(sha256: &Sha256) -> Self {
            sha256.clone().into()
        }
    }

    impl From<Sha256> for Sha256Digest {
        #[inline]
        fn from(sha256: Sha256) -> Self {
            Self::from(sha256.finalize())
        }
    }

    impl From<::digest::Output<Sha256>> for Sha256Digest {
        #[inline]
        fn from(output: ::digest::Output<Sha256>) -> Self {
            let output: [u8; 32] = *output.as_ref();
            Self(sha::Digest::from(output))
        }
    }

    impl BitXor<u32> for Sha256Digest {
        type Output = Self;

        #[inline]
        fn bitxor(mut self, rhs: u32) -> Self::Output {
            self ^= rhs;
            self
        }
    }

    impl BitXorAssign<u32> for Sha256Digest {
        #[inline]
        fn bitxor_assign(&mut self, rhs: u32) {
            self.0.as_mut_words()[0] ^= rhs;
        }
    }

    impl FromHex for Sha256Digest {
        type Error = crate::Error;
        fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error> {
            Ok(Self(sha::Digest::from_hex(hex).map_err(|e| {
                io::Error::new(io::ErrorKind::InvalidData, e)
            })?))
        }
    }

    #[inline]
    fn serialize_digest<W: io::Write>(digest: &sha::Digest, writer: &mut W) -> io::Result<()> {
        AsRef::<[u32; DIGEST_WORDS]>::as_ref(digest).serialize(writer)
    }

    #[inline]
    fn deserialize_digest<R: io::Read>(reader: &mut R) -> io::Result<sha::Digest> {
        <[u32; DIGEST_WORDS]>::deserialize_reader(reader).map(sha::Digest::new)
    }

    #[inline]
    fn serialize_receipt_meta<W: io::Write>(
        receipt_claim: &ReceiptClaim,
        writer: &mut W,
    ) -> io::Result<()> {
        let mut words = Vec::<u32>::new();
        receipt_claim
            .encode(&mut words)
            .map_err(|_| io::ErrorKind::InvalidData)?;
        words.serialize(writer)
    }

    #[inline]
    fn deserialize_receipt_meta<R: io::Read>(reader: &mut R) -> io::Result<ReceiptClaim> {
        let mut words = <VecDeque<u32>>::deserialize_reader(reader)?;
        ReceiptClaim::decode(&mut words).map_err(|_| io::ErrorKind::InvalidData.into())
    }

    pub type Sha256Pipe<T> = DigestPipe<Sha256, T>;

    /// A digest-ible borsh-encoded type.
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct BlockData<T> {
        /// Digest of the borsh-encoded journal.
        pub(crate) digest: RefCell<Sha256Digest>,
        pub(crate) t: T,
    }

    impl<T> BlockData<T> {
        pub fn digest(&self) -> Ref<Sha256Digest> {
            self.digest.borrow()
        }

        pub fn as_inner(&self) -> &T {
            &self.t
        }

        pub fn as_inner_mut(&mut self) -> &mut T {
            &mut self.t
        }

        pub fn into_inner(self) -> T {
            self.t
        }
    }

    impl<T: BorshSerialize> BlockData<T> {
        pub fn new(inner: T) -> Self {
            Self::new_with_bytes(inner, &mut Empty)
                .expect("should never fail to serialize and compute digest")
        }

        fn new_with_bytes<W: io::Write>(t: T, writer: &mut W) -> io::Result<Self> {
            let this = Self {
                digest: RefCell::new(Sha256Digest::ZERO),
                t,
            };
            this.serialize(writer)?;
            Ok(this)
        }
    }

    /// Initializes a default `T`, then computes its digest.
    impl<T: BorshSerialize + Default> Default for BlockData<T> {
        fn default() -> Self {
            Self::new(Default::default())
        }
    }

    impl<T: BorshSerialize> From<T> for BlockData<T> {
        fn from(other: T) -> Self {
            Self::new(other)
        }
    }

    // impl<T> Borrow<Sha256Digest> for BlockData<T> {
    //     fn borrow(&self) -> &Sha256Digest {
    //         self.digest.borrow().deref()
    //     }
    // }

    impl<T: BorshSerialize> BorshSerialize for BlockData<T> {
        fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
            let digest = Sha256Pipe::encode_to_writer(&self.t, writer)?;
            self.digest.replace(digest.into());
            Ok(())
        }
    }

    impl<T: BorshDeserialize> BorshDeserialize for BlockData<T> {
        fn deserialize_reader<R: io::Read>(reader: &mut R) -> io::Result<Self> {
            let (digest, decoded) = super::Sha256Pipe::decode_from_reader(reader)?;
            let digest = RefCell::new(digest.into());
            Ok(Self { digest, t: decoded })
        }
    }

    /// A borsh-encoded risc0 execution journal's outputs, prefixed by their
    /// associated [`ImageId`].
    ///
    #[derive(Clone, Debug, Eq, PartialEq, BorshDeserialize, BorshSerialize)]
    pub struct TypedJournal<T>(BlockData<(ImageId, T)>);

    impl<T> TypedJournal<T> {
        pub fn image_id(&self) -> &Sha256Digest {
            &self.0.t.0
        }

        pub fn digest(&self) -> Ref<Sha256Digest> {
            self.0.digest()
        }

        pub fn as_inner(&self) -> &T {
            &self.0.as_inner().1
        }

        pub fn as_inner_mut(&mut self) -> &mut T {
            &mut self.0.as_inner_mut().1
        }

        pub fn into_inner(self) -> T {
            self.0.into_inner().1
        }
    }

    impl<T: BorshSerialize> TypedJournal<T> {
        pub fn new(image_id: ImageId, inner: T) -> Self {
            Self(BlockData::new((image_id, inner)))
        }
    }

    impl<T: BorshSerialize + Default> TypedJournal<T> {
        pub fn default(image_id: ImageId) -> Self {
            Self(BlockData::new((image_id, Default::default())))
        }
    }

    impl<T: BorshSerialize + Default> Default for TypedJournal<T> {
        fn default() -> Self {
            Self::default(ImageId::default())
        }
    }

    // impl<T> Borrow<Sha256Digest> for TypedJournal<T> {
    //     fn borrow(&self) -> &Sha256Digest {
    //         &self.0.borrow()
    //     }
    // }

    #[cfg(target_os = "zkvm")]
    impl<T: BorshDeserialize> TypedJournal<T> {
        pub fn deserialize_verify_self<R: io::Read>(reader: &mut R) -> io::Result<Self> {
            let this = Self::deserialize_verify(reader)?;
            if this.image_id() != &self_image_id() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "journal is for another image_id",
                ));
            }
            Ok(this)
        }

        pub fn deserialize_verify<R: io::Read>(
            // image_id: &ImageId,
            reader: &mut R,
        ) -> io::Result<Self> {
            struct CopyReader<R>(R, Vec<u8>);
            impl<R> CopyReader<R> {
                fn from(reader: R) -> Self {
                    Self(reader, Vec::new())
                }
            }
            impl<R: io::Read> io::Read for CopyReader<R> {
                fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
                    let read = self.0.read(buf)?;
                    self.1.extend_from_slice(buf);
                    Ok(read)
                }
            }

            let (this, journal_bytes) = {
                let mut buf = CopyReader::from(reader);
                let inner = BlockData::deserialize_reader(&mut buf)?;
                (Self(inner), buf.1)
            };

            // let mut image_id_str = [0u32; 8];
            // risc0_zkvm::guest::env::env_var("IMAGE_ID", &mut image_id_str);
            // let image_id_str = crate::maybestd::str::from_utf8(&image_id_str[..])
            //     .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
            //     .parse_hex();

            // if strict && this.image_id() != image_id {
            //     return Err(io::Error::new(
            //         io::ErrorKind::InvalidData,
            //         "journal is for another image_id",
            //     ));
            // }
            trace(
                format_args!(
                    "verifying image_id {:?} and journal digest {:?}",
                    &this.image_id().0,
                    &this.digest(),
                ),
                None,
            );
            risc0_zkvm::guest::env::verify(this.image_id().0, journal_bytes.as_slice()).map_err(
                |e| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("invalid journal: {}", e),
                    )
                },
            )?;

            Ok(this)
        }
    }
}
