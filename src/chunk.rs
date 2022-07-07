//! Chunks of PNG file data
//!
//! Each chunk consists of four parts:
//! * `Length`
//! <br /> A **4-byte** unsigned integer giving the number of bytes in the chunk's data field.
//! The length counts only the data field, not itself, the chunk type code, or the CRC.
//!
//! * `Chunk Type`
//! <br /> See [Chunk Type](crate::chunk_type::ChunkType) for more details.
//!
//! * `Chunk Data`
//! <br /> The data bytes appropriate to the chunk type, if any. This field can be of zero length.
//!
//! * `CRC`
//! <br /> A **4-byte** [CRC](https://www.wikiwand.com/en/Cyclic_redundancy_check) (Cyclic Redundancy Check) calculated on the preceding bytes in the chunk,
//! including the chunk type code and chunk data fields, but not including the length field.

use std::fmt::Display;
use std::str::FromStr;

use crate::chunk_type::ChunkType;
use crate::{checksum_32, u8_4_from_slice};
use crate::{Error, Result, CHUNK_SIZE};

use crc::CRC_32_ISO_HDLC;

/// Chunk for a PNG file
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chunk {
    /// A 4-byte unsigned integer giving the number of bytes in the chunk’s data field.
    length: u32,
    /// A 4-byte chunk type code. [more](crate::chunk_type::ChunkType)
    chunk_type: ChunkType,
    /// The data bytes appropriate to the chunk type
    chunk_data: Vec<u8>,
    /// A 4-byte [CRC](https://www.wikiwand.com/en/Cyclic_redundancy_check) code for error-detecting
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, chunk_data: Vec<u8>) -> Chunk {
        let to_check = [&chunk_type.bytes(), chunk_data.as_slice()].concat();

        let crc = checksum_32(&CRC_32_ISO_HDLC, &to_check);

        Chunk {
            length: chunk_data.len() as u32,
            chunk_type,
            chunk_data,
            crc,
        }
    }

    pub fn from_strings(chunk_type: &str, data: &str) -> Result<Chunk> {
        let chunk_type = ChunkType::from_str(chunk_type)?;
        let data: Vec<u8> = data.bytes().collect();

        Ok(Chunk::new(chunk_type, data))
    }

    /// Return Length of chunk data
    pub fn length(&self) -> u32 {
        self.length
    }

    /// Return Type of chunk
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    /// Return Main Data of chunk
    pub fn data(&self) -> &[u8] {
        &self.chunk_data
    }

    /// Return CRC checksum
    pub fn crc(&self) -> u32 {
        self.crc
    }

    /// Return the data of chunk as Result<string>
    pub fn data_as_string(&self) -> Result<String> {
        String::from_utf8(self.data().to_vec()).map_err(Error::from)
    }

    /// Bytes representation for Chunk
    pub fn as_bytes(&self) -> Vec<u8> {
        [
            self.length.to_be_bytes().as_ref(),
            &self.chunk_type.bytes(),
            &self.chunk_data,
            self.crc.to_be_bytes().as_ref(),
        ]
        .concat()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        let length = u32::from_be_bytes(u8_4_from_slice(&bytes[0..CHUNK_SIZE]));

        if bytes.len() != (length as usize + 3 * CHUNK_SIZE) as usize {
            return Err(Error::Custom(
                "Chunk contains incorrect length information".to_owned(),
            ));
        }

        let chunk_type =
            ChunkType::try_from(u8_4_from_slice(&bytes[CHUNK_SIZE..2 * CHUNK_SIZE])).unwrap();

        let chunk_data = bytes[2 * CHUNK_SIZE..bytes.len() - CHUNK_SIZE].to_vec();

        let crc = u32::from_be_bytes(u8_4_from_slice(
            &bytes[bytes.len() - CHUNK_SIZE..bytes.len()],
        ));

        let to_check: Vec<u8> = [&chunk_type.bytes(), chunk_data.as_slice()].concat();

        if checksum_32(&CRC_32_ISO_HDLC, &to_check) != crc {
            Err(Error::Custom("CRC checksum fails".to_owned()))
        } else {
            Ok(Chunk {
                length,
                chunk_type,
                chunk_data,
                crc,
            })
        }
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chunk\n{{\n")?;
        write!(
            f,
            "\tlength: {}, chunk_type: {}\n\tdata: {:?}\n\tcrc: {}\n}}",
            self.length(),
            self.chunk_type(),
            self.data(),
            self.crc()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    #[test]
    fn crc_test() {
        let crc = checksum_32(&CRC_32_ISO_HDLC, b"123456789");
        println!("{:?}", crc);

        assert_eq!(crc, 0xcbf43926);
    }

    #[test]
    fn crc_test2() {
        let chunk_type = "RuSt";
        let message_bytes = "This is where your secret message will be!".as_bytes();

        let crc: u32 = 2882656334;
        let checked = checksum_32(
            &CRC_32_ISO_HDLC,
            &[chunk_type.as_bytes(), message_bytes].concat(),
        );

        println!("{}", checked);
        assert_eq!(checked, crc);
    }

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }

    #[test]
    fn test_chunk_display() {
        let chunk = testing_chunk();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);

        println!("{}", chunk);
    }

    #[test]
    fn test_chunk_as_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        assert_eq!(chunk.as_bytes(), chunk_data);
    }
}
