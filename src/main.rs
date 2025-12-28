mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use clap::Parser;
use crate::args::{Args::{self, Decode, Encode, Print, Remove}, DecodeArgs, EncodeArgs};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    // todo!()
    // let args = match Args::parse() {
    //     Decode(args) => Decode(args),
    //     Encode(args) => Encode(args),
    //     Remove(args) => Remove(args),
    //     Print(args) => Print(args)
    // };

    let args = Args::parse();
    match args {
        Encode(args) => EncodeArgs::handle(args),
        Decode(args) => DecodeArgs::handle(args),
        _ => Ok(())
    }
}
