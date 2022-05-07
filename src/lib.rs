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
pub mod chunk;
pub mod chunk_type;
pub mod commands;
pub mod error;
pub mod png;

pub use chunk::{checksum_32, Chunk};
pub use chunk_type::ChunkType;
pub use error::{Error, Result};

pub const CHUNK_SIZE: usize = 4;
