//! Parse command line arguments

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct PngChatArgs {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Encode the message in the specfic PNG file with a  certian type
    Encode(EncodeArgs),
    /// Decode the message in the specfic PNG file according to a certian chunk type
    Decode(DecodeArgs),
    /// Remove a message according to certian chunk type
    Remove(RemoveArgs),
    /// Print a list of PNG chunks that can be searched for messages
    Print(PrintArgs),
}

#[derive(Debug, Args, Clone)]
pub struct EncodeArgs {
    /// Input PNG file path
    pub file_path: PathBuf,
    /// Chunk Type
    pub chunk_type: String,
    /// Hideen message you want to put
    pub message: String,
    /// If set, save PNG with hidden message in a certian place
    pub output_file: Option<PathBuf>,
}

#[derive(Debug, Args, Clone)]
pub struct DecodeArgs {
    /// Input PNG file path
    pub file_path: PathBuf,
    /// Chunk Type
    pub chunk_type: String,
}

#[derive(Debug, Args, Clone)]
pub struct RemoveArgs {
    /// Input PNG file path
    pub file_path: PathBuf,
    /// Chunk Type
    pub chunk_type: String,
}

#[derive(Debug, Args, Clone)]
pub struct PrintArgs {
    /// Input PNG file path
    pub file_path: PathBuf,
}
