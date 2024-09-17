use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit},
    Aes256Gcm, Nonce, Key
};
use std::path::{Path, PathBuf};
use typenum;
use rprompt;
use std::fs;
use scrypt::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Scrypt,
    Params
};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let (hash, salt) = match (fs::read("hash.txt"), fs::read("salt.txt")) {
        (Ok(hash), Ok(salt)) => {
            println!("Hash and salt were found");
            println!("Proceeding with the encryption...\n");
            (hash, salt)
        }
        _ => {
            println!("Couldn't find your hash and salt, let's make a new one!");
            generate_hash_and_salt()?;

            // reread the files after making them
            (fs::read("hash.txt")?, fs::read("salt.txt")?)
        }
    };
    
    let key = derive_key(hash, salt)?;

    let answer = rprompt::prompt_reply("What file do you want to encrypt: ").unwrap();
    let file = answer.to_string();
    let content = fs::read(&file);

    let (ciphertext, nonce) = encrypt(content?, &key).expect("Encryption failed");

    let path = Path::new(&file);
    let mut output_path = PathBuf::from(path);
    output_path.set_extension("bin");

    let nonce_vec: Vec<u8> = nonce.as_slice().to_vec();

    let encrypted_data = (ciphertext, nonce_vec);

    fs::write(output_path, bincode::serialize(&encrypted_data)?)?;

    Ok(())
}

fn encrypt(content: Vec<u8>, key: &Key<Aes256Gcm>) -> Result<(Vec<u8>, Nonce<typenum::U12>), Box<dyn std::error::Error>> {
    // cipher instance with the key
    let cipher = Aes256Gcm::new(key);

    // gen a nonce
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    // encrypt content
    let ciphertext = cipher.encrypt(&nonce, &*content.as_ref()).expect("bruh");

    Ok((ciphertext, nonce))
}

//fn decrypt(ciphertext: Vec<u8>, nonce: Nonce, key: &Key) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
//
//}

fn derive_key(hash: Vec<u8>, salt: Vec<u8>) -> Result<Key<Aes256Gcm>, Box<dyn std::error::Error>> {
    let params: Params = Params::new(15, 8, 1, 32)?;

    let hash_str = String::from_utf8(hash)?;
    let salt_str = String::from_utf8(salt)?;

    let mut key_bytes = vec![0u8; 32];
    
    scrypt::scrypt(
        hash_str.as_bytes(),
        salt_str.as_bytes(),
        &params,
        &mut key_bytes
    )?;

    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);

    Ok(*key)
}

fn generate_hash_and_salt() -> Result<(), Box<dyn std::error::Error>> {
    // password prompt
    let binding = rprompt::prompt_reply("Enter your password: ").unwrap();
    
    // convert to bytes
    let password = binding.as_bytes();

    // gen a salt
    let salt = SaltString::generate(&mut OsRng);

    // make hash with salt
    let password_hash = Scrypt.hash_password(password, &salt)?;

    fs::write("hash.txt", password_hash.to_string())?;
    fs::write("salt.txt", salt.to_string())?;

    Ok(())
}
