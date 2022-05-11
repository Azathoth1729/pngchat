use clap::Parser;

use pngchat::{
    args::{Commands, PngChatArgs},
    commands::{decode, encode, print_chunks, remove},
    Result,
};

fn main() -> Result<()> {
    let cli = PngChatArgs::parse();

    match &cli.command {
        Commands::Encode(args) => encode(args),
        Commands::Decode(args) => decode(args),
        Commands::Remove(args) => remove(args),
        Commands::Print(args) => print_chunks(args),
    }
}
