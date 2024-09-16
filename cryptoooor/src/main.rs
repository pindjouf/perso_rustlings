use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key // Or `Aes128Gcm`
};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key = Aes256Gcm::generate_key(OsRng);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let cipher = Aes256Gcm::new(&key);

    let message = load_file("message.txt").expect("damn");

    let ciphertext = cipher.encrypt(&nonce, message.as_ref()).expect("can't encrypt");
    let _ = fs::write("message.bin", ciphertext);

    let cryptoFile = load_file("message.bin").expect("damn");
    let plaintext = cipher.decrypt(&nonce, cryptoFile.as_ref()).expect("can't decrypt your file");
    let _ = fs::write("decrypted.txt", plaintext);

    Ok(())
}

fn load_file(file: &str) -> Result<Vec<u8>, std::io::Error> {
    fs::read(file)
}
