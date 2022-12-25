use super::sha512_n;
use hex::decode;
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub fn exercise_5_3(bytes: usize, times: usize) {
    let mut timings = vec![];
    for _ in 1..times {
        let mut hashes = HashMap::new();
        let start = Instant::now();
        let (value1, value2, hash) = (1usize..1_000_000)
            .find_map(|i| {
                let hash = sha512_n(&i.to_be_bytes(), bytes);
                hashes.insert(hash.clone(), i).map(|i2| (i, i2, hash))
            })
            .unwrap();
        timings.push(start.elapsed());
        println!(
            "Collision with {:?} and {:?} with hash {:?}",
            value1, value2, hash
        );
    }

    println!(
        "Average time for collision: {:?}",
        timings
            .iter()
            .sum::<Duration>()
            .div_f32(timings.len() as f32)
    );
}

pub fn exercise_5_4(bytes: usize, message: &str, times: usize) {
    let msg = match decode(message) {
        Ok(msg) => {
            if msg.len() != bytes {
                eprintln!(
                    "Expected message with {} bytes, got {} bytes",
                    bytes,
                    msg.len()
                );
                return;
            }
            msg
        }
        Err(error) => {
            eprintln!("Error: {}", error);
            return;
        }
    };
    let mut runs = vec![];
    for _ in 1..times {
        let iterations = (1usize..100_000_000)
            .find_map(|i| {
                let hash = sha512_n(&i.to_be_bytes(), bytes);
                if hash == msg {
                    Some(i)
                } else {
                    None
                }
            })
            .unwrap();
        runs.push(iterations);
    }

    println!(
        "Average number of iterations: {}",
        runs.iter().sum::<usize>() as f32 / runs.len() as f32
    );
}
