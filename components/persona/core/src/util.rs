pub use self::digest::DigestPipe;
pub use risc0::{Sha256Digest, Sha256Pipe};

use crate::{
    device::DeviceKey,
    maybestd::{io, ops, vec::Vec},
};
use borsh::{BorshDeserialize, BorshSerialize};
use risc0_zkvm::sha::{rust_crypto::Sha256, DIGEST_BYTES, DIGEST_WORDS};

// pub mod cid {
//     use super::*;
//     use ::cid::Cid;

//     pub fn serialize_cid<W: io::Write>(cid: &Cid, writer: &mut W) -> io::Result<()> {
//         let mut bytes = Vec::with_capacity(cid.encoded_len());
//         cid.write_bytes(bytes.as_mut_slice())
//             .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}", e)))?;
//         bytes.serialize(writer)
//     }

//     pub fn deserialize_cid<R: io::Read>(reader: &mut R) -> io::Result<Cid> {
//         let mut bytes = Vec::with_capacity();
//         Cid::read_bytes(reader).map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}", e)))
//     }
// }

pub mod digest {
    use super::*;
    use ::digest::Digest;

    pub struct DigestPipe<D, T>(D, T);

    impl<D: Digest, T> From<T> for DigestPipe<D, T> {
        fn from(pipe: T) -> Self {
            Self(D::new(), pipe)
        }
    }

    impl<D: Digest, R: io::Read> DigestPipe<D, R> {
        pub fn decode_from_reader<T: BorshDeserialize>(reader: R) -> io::Result<(D, T)> {
            let mut this = Self::from(reader);
            let outputs = T::deserialize_reader(&mut this)?;
            Ok((this.0, outputs))
        }
    }

    impl<D: Digest, R: io::Read> io::Read for DigestPipe<D, R> {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let read = self.1.read(buf)?;
            self.0.update(&buf[..read]);
            Ok(read)
        }
    }

    impl<D: Digest, W: io::Write> DigestPipe<D, W> {
        pub fn encode_to_writer<T: BorshSerialize>(val: &T, writer: W) -> io::Result<D> {
            let mut this = Self::from(writer);
            val.serialize(&mut this)?;
            Ok(this.0)
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
    use ed25519_dalek::{Signature, PUBLIC_KEY_LENGTH};

    pub fn serialize_device_key<W: io::Write>(
        device_key: &DeviceKey,
        writer: &mut W,
    ) -> io::Result<()> {
        device_key.to_bytes().serialize(writer)
    }

    pub fn deserialize_device_key<R: io::Read>(reader: &mut R) -> io::Result<DeviceKey> {
        let pk_bytes = <[u8; PUBLIC_KEY_LENGTH]>::deserialize_reader(reader)?;
        DeviceKey::from_bytes(&pk_bytes)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid public key"))
    }

    pub fn serialize_signature<W: io::Write>(
        signature: &Signature,
        writer: &mut W,
    ) -> io::Result<()> {
        signature.to_bytes().serialize(writer)
    }

    pub fn deserialize_signature<R: io::Read>(reader: &mut R) -> io::Result<Signature> {
        <[u8; Signature::BYTE_SIZE]>::deserialize_reader(reader)
            .map(|bytes| Signature::from_bytes(&bytes))
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
        let len = u32::deserialize_reader(reader)?;
        (0..len)
            .map(|_| deserialize_signature(reader))
            .collect::<io::Result<Vec<_>>>()
    }
}

pub mod risc0 {
    use super::*;
    // use alloc::collections::VecDeque;
    use ::digest::Digest;
    use risc0_zkvm::sha;

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

    impl AsRef<[u32; DIGEST_WORDS]> for Sha256Digest {
        #[inline]
        fn as_ref(&self) -> &[u32; DIGEST_WORDS] {
            self.0.as_ref()
        }
    }

    impl AsRef<[u8; DIGEST_BYTES]> for Sha256Digest {
        #[inline]
        fn as_ref(&self) -> &[u8; DIGEST_BYTES] {
            self.0.as_ref()
        }
    }

    impl AsRef<[u8]> for Sha256Digest {
        #[inline]
        fn as_ref(&self) -> &[u8] {
            self.0.as_ref()
        }
    }

    impl From<&Sha256> for Sha256Digest {
        #[inline]
        fn from(sha256: &Sha256) -> Self {
            Self::from(sha256.clone())
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

    #[inline]
    fn serialize_digest<W: io::Write>(digest: &sha::Digest, writer: &mut W) -> io::Result<()> {
        digest.as_words().serialize(writer)
    }

    #[inline]
    fn deserialize_digest<R: io::Read>(reader: &mut R) -> io::Result<sha::Digest> {
        <[u32; DIGEST_WORDS]>::deserialize_reader(reader).map(sha::Digest::new)
    }

    // pub fn serialize_receipt_meta<W: io::Write>(
    //     receipt_meta: &ReceiptMeta,
    //     writer: &mut W,
    // ) -> io::Result<()> {
    //     let mut words = Vec<u32>::new();
    //     receipt_meta.encode(&mut words).map_err(|_| io::ErrorKind::InvalidData.into())?;
    //     words.serialize(writer)
    // }

    // pub fn deserialize_receipt_meta<R: io::Read>(
    //     reader: &mut R,
    // ) -> io::Result<ReceiptMetadata> {
    //     let mut words = <VecDeque<u32>>::deserialize_reader(reader)?;
    //     ReceiptMetadata::decode(&mut words).map_err(|_| io::ErrorKind::InvalidData.into())
    // }

    pub type Sha256Pipe<T> = DigestPipe<Sha256, T>;

    /// An borsh-encoded risc0 execution journal's outputs.
    #[derive(Clone, Debug, Default)]
    pub struct TypedJournal<T> {
        /// Digest of the borsh-encoded journal.
        digest: Sha256Digest,
        outputs: T,
    }

    impl<T> TypedJournal<T> {
        pub fn digest(&self) -> &Sha256Digest {
            &self.digest
        }

        pub fn into(self) -> T {
            self.outputs
        }
    }

    impl<T> AsRef<T> for TypedJournal<T> {
        fn as_ref(&self) -> &T {
            &self.outputs
        }
    }

    impl<T: BorshSerialize> BorshSerialize for TypedJournal<T> {
        fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
            self.outputs.serialize(writer)
        }
    }

    impl<T: BorshDeserialize> BorshDeserialize for TypedJournal<T> {
        fn deserialize_reader<R: io::Read>(reader: &mut R) -> io::Result<Self> {
            struct BufReader<R>(R, Vec<u8>);
            impl<R> BufReader<R> {
                fn from(reader: R) -> Self {
                    Self(reader, Vec::new())
                }
            }
            impl<R: io::Read> io::Read for BufReader<R> {
                fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
                    let read = self.0.read(buf)?;
                    self.1.extend_from_slice(buf);
                    Ok(read)
                }
            }

            #[cfg(not(target_os = "zkvm"))]
            let (digest, outputs) = {
                let (digest, outputs) = super::Sha256Pipe::decode_from_reader(reader)?;
                (digest.into(), outputs)
            };

            #[cfg(target_os = "zkvm")]
            let (digest, outputs) = {
                let (digest, journal_bytes, outputs) = {
                    let mut buf = BufReader::from(reader);
                    let (digest, outputs) = super::Sha256Pipe::decode_from_reader(&mut buf)?;
                    (digest.into(), buf.1, outputs)
                };

                // in the zkvm, verify the journal by default
                let image_id = [0u8; 32].into();
                risc0_zkvm::guest::env::verify(image_id, journal_bytes.as_slice()).map_err(
                    |e| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("invalid journal: {}", e),
                        )
                    },
                )?;

                (digest, outputs)
            };

            Ok(TypedJournal { digest, outputs })
        }
    }
}
