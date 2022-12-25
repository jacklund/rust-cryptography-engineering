use crate::{homework_1::*, homework_3::*};
use clap::{Parser, Subcommand};
use ring::digest;

pub mod homework_1;
pub mod homework_3;

fn sha512_n(value: &[u8], bytes: usize) -> Vec<u8> {
    digest::digest(&digest::SHA512, value).as_ref()[..bytes].into()
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Subcommand)]
enum Exercises {
    VignereEncrypt {
        key: String,
        plaintext: String,
    },
    VignereDecrypt {
        key: String,
        ciphertext: String,
    },
    #[command(name = "5.3")]
    E5_3 {
        bytes: usize,
        times: usize,
    },

    #[command(name = "5.4")]
    E5_4 {
        bytes: usize,
        message: String,
        times: usize,
    },

    Blake3Bench,
}

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    exercise: Exercises,
}

fn main() {
    let args = Args::parse();

    match args.exercise {
        Exercises::VignereEncrypt { key, plaintext } => vignere::encrypt(&key, &plaintext),
        Exercises::VignereDecrypt { key, ciphertext } => vignere::decrypt(&key, &ciphertext),
        Exercises::E5_3 { bytes, times } => exercise_5_3(bytes, times),
        Exercises::E5_4 {
            bytes,
            message,
            times,
        } => exercise_5_4(bytes, &message, times),
        Exercises::Blake3Bench => {
            println!("Run 'cargo bench'");
        }
    }
}
