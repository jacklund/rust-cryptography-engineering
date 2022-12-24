use crate::chapter_5::exercise_5_3;
use clap::{Parser, Subcommand};
use ring::digest;

pub mod chapter_5;

fn sha512_n(value: &[u8], bytes: usize) -> Vec<u8> {
    digest::digest(&digest::SHA512, value).as_ref()[..bytes].into()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Subcommand)]
enum Exercises {
    #[command(name = "5.3")]
    E5_3 { bytes: usize, times: usize },
}

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    exercise: Exercises,
}

fn main() {
    let args = Args::parse();

    match args.exercise {
        Exercises::E5_3 { bytes, times } => exercise_5_3(bytes, times),
    }
}
