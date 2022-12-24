use super::sha512_n;
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
