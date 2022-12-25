pub mod vignere {
    pub fn encrypt(key: &str, plaintext: &str) {
        let utf8 = plaintext
            .to_lowercase()
            .as_bytes()
            .iter()
            .zip(key.to_lowercase().bytes().cycle())
            .map(|(p, k)| ((((p - b'a') + (k - b'a')) % 26) + b'a') as u8)
            .collect::<Vec<u8>>();
        let ciphertext = match std::str::from_utf8(&utf8) {
            Ok(string) => string,
            Err(error) => {
                eprintln!("Error converting utf-8 string: {}", error);
                return;
            }
        };

        println!(
            "key = {}, plaintext = {}, ciphertext = {}",
            key, plaintext, ciphertext
        );
    }

    pub fn decrypt(key: &str, ciphertext: &str) {
        let utf8 = ciphertext
            .to_lowercase()
            .as_bytes()
            .iter()
            .zip(key.to_lowercase().bytes().cycle())
            .map(|(c, k)| (((26 + (c - b'a') - (k - b'a')) % 26) + b'a') as u8)
            .collect::<Vec<u8>>();
        let plaintext = match std::str::from_utf8(&utf8) {
            Ok(string) => string,
            Err(error) => {
                eprintln!("Error converting utf-8 string: {}", error);
                return;
            }
        };

        println!(
            "key = {}, ciphertext = {}, plaintext = {}",
            key, ciphertext, plaintext
        );
    }
}
