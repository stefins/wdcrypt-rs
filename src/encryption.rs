#![allow(dead_code)]

const FERNET_FILE: &str = ".secret.key";
use fernet;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;
use std::str;

// This function will encrypt a string to ciphertext using Fernet
pub fn encrypt_to_cipher(key: &String, content: &[u8]) -> String {
    let fernet = fernet::Fernet::new(&key).unwrap();
    fernet.encrypt(content)
}

// This function will decrypt a ciphertext  to normal form using Fernet
pub fn decrypt_to_normal(
    key: &String,
    ciphertext: &String,
) -> Result<Vec<u8>, fernet::DecryptionError> {
    let fernet = fernet::Fernet::new(&key).unwrap();
    match fernet.decrypt(&ciphertext) {
        Ok(result) => Ok(result),
        Err(err) => {
            return Err(err);
        }
    }
}

// This function write the Fernet key to .secret.key
pub fn write_fernet_key_to_file(key: &String) {
    if Path::new(FERNET_FILE).exists() {
        println!("{} already exists [Aborting]", FERNET_FILE);
        process::exit(1);
    }
    let mut file = File::create(FERNET_FILE).unwrap();
    file.write_all(&key.as_bytes()).unwrap();
}

// This function will read the fernet key from file
pub fn read_fernet_key_from_file() -> String {
    if !Path::new(FERNET_FILE).exists() {
        println!("{} doesn't exist", FERNET_FILE);
        process::exit(1);
    }
    let mut file = File::open(FERNET_FILE).unwrap();
    let mut key = String::new();
    file.read_to_string(&mut key).unwrap();
    key
}
