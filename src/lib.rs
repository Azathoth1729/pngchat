//! Encrypted communication using PNG images.
//!
//! Inspired from by [PNGme: An Intermediate Rust Project](https://picklenerd.github.io/pngme_book/introduction.html)
//!
//! # Goal
//! Making a command line program that lets you hide secret messages in PNG files.
//!
//! The main tasks of `pngchat` are:
//! * Encode a message into a PNG file
//! * Decode a message stored in a PNG file
//! * Remove a message from a PNG file
//! * Print a list of PNG chunks that can be searched for messages
//!
//! # PNG File Structure
//! See the [PNG file structure spec](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html) for more details

pub mod args;
mod chunk;
mod chunk_type;
mod commands;
mod error;
pub mod png;

pub use chunk::Chunk;
pub use chunk_type::ChunkType;
pub use png::Png;

pub use error::{Error, Result};

use crc::{Algorithm, Crc};

///  Compute CRC32 using certian algorithm
pub(crate) fn checksum_32(algo: &'static Algorithm<u32>, bytes: &[u8]) -> u32 {
    let crc = Crc::<u32>::new(&algo);
    crc.checksum(bytes)
}

/// Slice to Array(u8) of 4 elements
pub(crate) fn u8_4_from_slice(arr: &[u8]) -> [u8; CHUNK_SIZE] {
    arr.try_into().expect("Invalid slice length")
}

/// 4 bytes size
pub const CHUNK_SIZE: usize = 4;
