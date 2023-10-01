use crate::device::DeviceKey;
use borsh::{io, BorshDeserialize, BorshSerialize};

pub mod cid {
    use super::*;
    use ::cid::Cid;

    pub fn serialize_cid<W: io::Write>(cid: &Cid, writer: &mut W) -> io::Result<()> {
        cid.write_bytes(writer)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(())
    }

    pub fn deserialize_cid<R: io::Read>(reader: &mut R) -> io::Result<Cid> {
        Cid::read_bytes(reader).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
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
        signature.to_bytes().serialize(writer)?;
        Ok(())
    }

    // pub fn deserialize_signature<R: io::Read>(reader: &mut R) -> io::Result<Signature> {
    //     let bytes = <[u8; Signature::BYTE_SIZE]>::deserialize_reader(reader)?;
    //     Ok(Signature::from_bytes(&bytes))
    // }

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
        let bytes = <Vec<[u8; Signature::BYTE_SIZE]>>::deserialize_reader(reader)?;
        Ok(bytes
            .into_iter()
            .map(|bytes| Signature::from_bytes(&bytes))
            .collect::<Vec<_>>())
    }
}

pub mod risc0 {
    use super::*;
    use risc0_zkvm::Receipt;

    pub fn serialize_receipt<W: io::Write>(receipt: &Receipt, writer: &mut W) -> io::Result<()> {
        bincode::serialize_into(writer, receipt)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(())
    }

    pub fn deserialize_receipt<R: io::Read>(reader: &mut R) -> io::Result<Receipt> {
        let receipt = bincode::deserialize_from(reader)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(receipt)
    }
}
