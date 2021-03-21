use std::io::Error;

use crate::file_utils;

pub struct File<'a> {
    name: &'a str,
    fernet_key: String,
}

impl<'a> File<'a> {
    pub fn new(name: &'a str, fernet_key: String) -> Self {
        Self { name, fernet_key }
    }
    pub fn encrypt(&self) {
        file_utils::encrypt_file(&self.name, &self.fernet_key).unwrap();
    }

    pub fn decrypt(&self) -> Result<(),()> {
        match file_utils::decrypt_file(&self.name, &self.fernet_key){
            Ok(_) => Ok(()),
            Err(err) =>{
                eprintln!("Error, {}",err);
                Err(())
            }
        }
    }
}

pub struct Folder<'a> {
    name: &'a str,
}

impl<'a> Folder<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name: &name }
    }
    pub fn tar(&self) -> Result<(),Error> {
        match file_utils::create_tar_gz(&self.name){
            Ok(_) => Ok(()),
            Err(err) => Err(err)
        }
    }
    #[allow(dead_code)]
    fn untar(&self) {
        todo!()
    }
}
