//! Some utility functions

use crc::{Algorithm, Crc};

use crate::CHUNK_SIZE;

///  Compute CRC32 using certian algorithm
pub fn checksum_32(algo: &'static Algorithm<u32>, bytes: &[u8]) -> u32 {
    let crc = Crc::<u32>::new(&algo);
    crc.checksum(bytes)
}

/// Slice to Array(u8) of 4 elements
pub fn u8_4_from_slice(arr: &[u8]) -> [u8; CHUNK_SIZE] {
    arr.try_into().expect("Invalid slice length")
}
