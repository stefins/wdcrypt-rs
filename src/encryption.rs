#![allow(dead_code)]

const FERNET_FILE: &str = ".secret.key";
use crate::utils::ask_bool;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;
use std::str;

// This function will encrypt a string to ciphertext using Fernet
pub fn encrypt_to_cipher(key: &str, content: &[u8]) -> String {
    let fernet = fernet::Fernet::new(&key).unwrap();
    fernet.encrypt(content)
}

// This function will decrypt a ciphertext  to normal form using Fernet
pub fn decrypt_to_normal(key: &str, ciphertext: &str) -> Result<Vec<u8>, fernet::DecryptionError> {
    let fernet = fernet::Fernet::new(&key).unwrap();
    match fernet.decrypt(&ciphertext) {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

// This function write the Fernet key to .secret.key
pub fn write_fernet_key_to_file(key: String) -> String {
    if Path::new(FERNET_FILE).exists() {
        println!("{} already exists", FERNET_FILE);
        if ask_bool("Do you want to use the existing key?", false).unwrap() {
            return read_fernet_key_from_file();
        }
        process::exit(1);
    }
    let mut file = File::create(FERNET_FILE).unwrap();
    file.write_all(&key.as_bytes()).unwrap();
    key
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

#[cfg(test)]
mod tests {
    use super::{
        decrypt_to_normal, encrypt_to_cipher, read_fernet_key_from_file, write_fernet_key_to_file,
    };
    use std::fs;

    fn clean_file() {
        fs::remove_file(".secret.key").unwrap();
    }

    #[test]
    fn encryption_test() {
        let key = "IVijuDdvEix5PnxKP9_4VioeeZVCtRiLWruh3ynM6og=".to_string();
        let test_cipher_text = encrypt_to_cipher(&key, &"hello world".as_bytes());
        let dec_cipher_text = decrypt_to_normal(&key, &test_cipher_text).unwrap();
        assert_eq!(dec_cipher_text, "hello world".as_bytes());
    }

    #[test]
    fn key_file_test() {
        let key = "IVijuDdvEix5PnxKP9_4VioeeZVCtRiLWruh3ynM6og=".to_string();
        write_fernet_key_to_file(key.clone());
        assert_eq!(read_fernet_key_from_file(), key);
        clean_file();
    }
}
