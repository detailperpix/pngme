use clap::Parser;
use std::{fs, str::FromStr};

use crate::{
    chunk::Chunk,
    chunk_type::ChunkType,
    png::{Png, PngParseError},
    Error,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub enum Args {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(clap::Args, Debug)]
#[command(version, about, long_about=None)]
pub struct EncodeArgs {
    filepath: std::path::PathBuf,

    chunk_type: String,

    message: String,

    output_filepath: Option<std::path::PathBuf>,
}

#[derive(clap::Args, Debug)]
pub struct DecodeArgs {
    filepath: std::path::PathBuf,
    chunk_type: String,
}

#[derive(clap::Args, Debug)]
pub struct RemoveArgs {
    filepath: std::path::PathBuf,
    chunk_type: String,
}

#[derive(clap::Args, Debug)]
pub struct PrintArgs {
    filepath: std::path::PathBuf,
}

impl EncodeArgs {
    pub fn handle_encode(self) -> Result<(), Error> {
        let png_bin = fs::read(&self.filepath)?;
        let mut png = Png::try_from(png_bin.as_slice())?;

        let chunk_type = ChunkType::from_str(&self.chunk_type)?;

        let new_chunk = Chunk::new(chunk_type, String::into_bytes(self.message));

        png.append_chunk(new_chunk);

        if let Some(output_file) = self.output_filepath {
            fs::write(output_file, png.as_bytes())?;
        } else {
            fs::write(&self.filepath, png.as_bytes())?;
        }
        Ok(())
    }
}
