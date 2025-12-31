use clap::Parser;
use pngme::args::{
    Args::{self, Decode, Encode, Print, Remove},
    DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs,
};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

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
        Remove(args) => RemoveArgs::handle(args),
        Print(args) => PrintArgs::handle(args),
    }
}
