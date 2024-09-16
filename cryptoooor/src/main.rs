use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit},
    Aes256Gcm, Nonce, Key
};
use rprompt;
use std::fs;
use std::io::{self, Write, Read};
use scrypt::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Scrypt,
    Params
};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let _ = derive_key();
    Ok(())
}

fn encrypt(content: Vec<u8>, key: &Key) -> Result<(Vec<u8>, Nonce), Box<dyn std::error::Error>> {
    let cipher = Aes256Gcm::new(key);

    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, &*content.as_ref())?;
}

//fn decrypt(ciphertext: Vec<u8>, nonce: Nonce, key: &Key) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
//
//}

//fn read_encrypted_file(file_path: &str) -> Result<(Nonce, Vec<u8>), io::Error> {
//
//}
//
fn derive_key() -> Result<(), Box<dyn std::error::Error>> {
    // password prompt
    let binding = rprompt::prompt_reply("Enter your password: ").unwrap();
    
    // convert to bytes
    let password = binding.as_bytes();

    // gen a salt
    let salt = SaltString::generate(&mut OsRng);

    // 
    let password_hash = Scrypt.hash_password(password, &salt)?;

    fs::write("hash.txt", password_hash.to_string())?;
    Ok(())
}
