//! Hide messages in the PNG file.
//!
//! The intent of this little project is to learn how to encode PNG file and add some messages inside it
//!
//! Idea come from [PNGme: An Intermediate Rust Project](https://picklenerd.github.io/pngme_book/introduction.html)
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
//! # Getting Started
//! > Note:If you don't install rust toolchain, install it first from [rustup](https://rustup.rs/)
//! 
//! First you need to make sure your rust toolchain is up to date. For the latest run
//! ```bash
//! rustup update
//! ```
//!
//! Then you need to git clone the repo and then [cargo install local binary](https://doc.rust-lang.org/cargo/commands/cargo-install.html)
//!
//! ```bash
//! git clone https://github.com/Azathoth1729/pngchat.git
//! cargo install --path .
//! ```
//!
//! Now you should be able to run commands `pngchat` directly on your terminal like:
//! 
//! ```bash
//! pngchat 1.0.1
//! Azathoth
//! Hide messages in the PNG file
//! 
//! USAGE:
//!     pngchat <SUBCOMMAND>
//! 
//! OPTIONS:
//!     -h, --help       Print help information
//!     -V, --version    Print version information
//! 
//! SUBCOMMANDS:
//!     decode    Decode the message in the specfic PNG file according to a certian chunk type
//!     encode    Encode the message in the specfic PNG file with a  certian type
//!     help      Print this message or the help of the given subcommand(s)
//!     print     Print a list of PNG chunks that can be searched for messages
//!     remove    Remove a message according to certian chunk type
//! ```
//! 
//! # Uasge
//!
//! ```bash
//! # Encodes a message into a PNG file and saves the result
//! pngchat encode ./test.png ruSt "This is a hidden message"
//!
//! # Searches for a message hidden in a PNG file and prints the message if one is found
//! pngchat decode ./test.png ruSt
//!
//! # Removes a chunk from a PNG file and saves the result
//! pngchat remove ./test.png ruSt
//!
//! # Prints all of the chunks in a PNG file
//! pngchat print ./test.png
//! ```
//! 
//! # Links
//! See the [PNG file structure spec](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html) for more details about how PNG file structured
//! 
//! # Uninstalling
//! Simply run if you install the binary from `cargo install`
//! ```bash
//! cargo uninstall pngchat
//! ```

pub mod args;
pub mod commands;

mod chunk;
mod chunk_type;
mod png;

mod error;
mod utils;

pub use error::{Error, Result};
pub use png::Png;
pub use utils::{checksum_32, u8_4_from_slice};

/// 4 bytes size
pub const CHUNK_SIZE: usize = 4;
