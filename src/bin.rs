use ring::digest;

pub mod chapter_5;

fn sha512_n(value: &[u8], bytes: usize) -> Vec<u8> {
    digest::digest(&digest::SHA512, value).as_ref()[..bytes].into()
}

#[main]
fn main() {
}
