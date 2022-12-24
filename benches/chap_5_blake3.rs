use blake3;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ring::digest;
use sha3::{Digest, Sha3_256};

const TEXT: &[u8] = b"some text";

pub fn bench_blake3(c: &mut Criterion) {
    c.bench_function("blake3", |b| b.iter(|| blake3::hash(black_box(TEXT))));
}

pub fn bench_sha2(c: &mut Criterion) {
    c.bench_function("sha2", |b| {
        b.iter(|| digest::digest(&digest::SHA256, black_box(TEXT)))
    });
}

pub fn bench_sha3(c: &mut Criterion) {
    c.bench_function("sha3", |b| {
        b.iter(|| {
            let mut hasher = Sha3_256::new();
            hasher.update(black_box(TEXT));
            hasher.finalize();
        })
    });
}

criterion_group!(blake3_bench, bench_blake3, bench_sha2, bench_sha3);
criterion_main!(blake3_bench);
