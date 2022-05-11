//! Functions for command line usage

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::png::{Chunk, Png};
use crate::{Error, Result};

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: &EncodeArgs) -> Result<()> {
    let mut png = Png::from_file(&args.file_path)?;
    let chunk = Chunk::from_strings(&args.chunk_type, &args.message)?;
    png.append_chunk(chunk);

    if let Some(output_file) = &args.output_file {
        png.wrtie_file(output_file)
    } else {
        png.wrtie_file(&args.file_path)
    }
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: &DecodeArgs) -> Result<()> {
    let png = Png::from_file(&args.file_path)?;

    if let Some(chunk) = png.chunk_by_type(&args.chunk_type) {
        println!("msg: {}", chunk.data_as_string()?);
        Ok(())
    } else {
        // Err(Error::Custom(&msg))
        Err(Error::Custom(format!(
            "This file does not contain msg of chunk type {}",
            args.chunk_type
        )))
    }
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: &RemoveArgs) -> Result<()> {
    let mut png = Png::from_file(&args.file_path)?;
    png.remove_chunk(&args.chunk_type)?;
    png.wrtie_file(&args.file_path)
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: &PrintArgs) -> Result<()> {
    let png = Png::from_file(&args.file_path)?;
    println!(
        "File: {}, Size: {}",
        &args.file_path.display(),
        png.as_bytes().len()
    );

    for (i, chunk) in png.chunks().iter().enumerate() {
        println!(
            "  chunk#{}{{ chunk_type: {}, data_length: {}}}",
            i,
            chunk.chunk_type(),
            chunk.length(),
        );
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use std::path::PathBuf;

    lazy_static! {
        static ref IMG_PATH: PathBuf = [".", "assets", "imgs"].iter().collect();
    }


    fn testing_origin_path() -> PathBuf {
        let mut path = IMG_PATH.to_path_buf();
        path.push("test.png");
        path
    }

    fn testing_out_path() -> PathBuf {
        let mut path = IMG_PATH.to_path_buf();
        path.push("test_out.png");
        path
    }

    fn testing_args() -> (PathBuf, String, String, PathBuf) {
        let file_path = testing_origin_path();
        let chunk_type = String::from("ruSt");
        let message = String::from("This is a secret message!");
        let output_file = testing_out_path();

        (file_path, chunk_type, message, output_file)
    }

    #[ignore]
    #[test]
    fn test_read_png() {
        let png = Png::from_file(testing_origin_path()).unwrap();
        let png_btyes = png.as_bytes();
        let chunks = png.chunks();

        println!(
            "png.len() = {}\npng.chunks.len() = {}",
            png_btyes.len(),
            chunks.len()
        );

        println!();
        for chunk in chunks {
            println!(
                "chunk_type: {}, data_length: {}",
                chunk.chunk_type(),
                chunk.length(),
            );
        }
    }

    #[ignore]
    #[test]
    fn test_encode_command() {
        let (file_path, chunk_type, message, _) = testing_args();

        let encode_args = EncodeArgs {
            file_path: file_path.clone(),
            chunk_type: chunk_type.clone(),
            message,
            output_file: None,
        };

        let remove_args = RemoveArgs {
            file_path: file_path.clone(),
            chunk_type: chunk_type.clone(),
        };

        let print_origin_arg = PrintArgs {
            file_path: file_path.clone(),
        };

        println!("Before encoding...");
        print_chunks(&print_origin_arg).unwrap();

        encode(&encode_args).unwrap();
        println!("After encoding...");
        print_chunks(&print_origin_arg).unwrap();

        remove(&remove_args).unwrap();
    }

    #[ignore]
    #[test]
    fn test_encode_and_decode_command() {
        let (file_path, chunk_type, message, output_file) = testing_args();

        let encode_args = EncodeArgs {
            file_path: file_path.clone(),
            chunk_type: chunk_type.clone(),
            message,
            output_file: Some(output_file.clone()),
        };

        let decode_args = DecodeArgs {
            file_path: output_file.clone(),
            chunk_type: chunk_type.clone(),
        };

        let remove_args = RemoveArgs {
            file_path: output_file.clone(),
            chunk_type: chunk_type.clone(),
        };

        encode(&encode_args).unwrap();
        decode(&decode_args).unwrap();
        remove(&remove_args).unwrap();
    }

    #[ignore]
    #[test]
    fn test_remove_command() {
        let (file_path, chunk_type, message, output_file) = testing_args();

        let encode_args = EncodeArgs {
            file_path: file_path.clone(),
            chunk_type: chunk_type.clone(),
            message,
            output_file: Some(output_file.clone()),
        };

        let remove_args = RemoveArgs {
            file_path: output_file.clone(),
            chunk_type: chunk_type.clone(),
        };

        let print_out_args = PrintArgs {
            file_path: output_file.clone(),
        };

        encode(&encode_args).unwrap();
        print_chunks(&print_out_args.clone()).unwrap();
        println!("Removeing chunk...");
        remove(&remove_args).unwrap();
        print_chunks(&print_out_args).unwrap();

        encode(&encode_args).unwrap();
    }

    #[ignore]
    #[test]
    fn test_print_chunks_command() {
        let (file_path, _, _, output_file) = testing_args();

        let print_origin_arg = PrintArgs {
            file_path: file_path.clone(),
        };

        let print_out_args = PrintArgs {
            file_path: output_file.clone(),
        };

        print_chunks(&print_origin_arg).unwrap();
        print_chunks(&print_out_args).unwrap();
    }
}
