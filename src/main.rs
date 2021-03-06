mod encryption;
mod file_utils;
use clap::{App, AppSettings, Arg};

fn main() {
    let matches = App::new("wdcrypt")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("0.1.0")
        .author("Stef stefin2016@gmail.com")
        .about("Encrypt your current working directory")
        .arg(
            Arg::new("encrypt")
                .about("Encrypt the current working directory")
                .short('e')
                .long("encrypt"),
        )
        .arg(
            Arg::new("decrypt")
                .about("Decrypt the current working directory with key")
                .short('d')
                .long("decrypt"),
        )
        .get_matches();
    if matches.is_present("encrypt") {
        file_utils::tar_all_folders().unwrap();
        file_utils::encrypt_all_files().unwrap();
    }
    if matches.is_present("decrypt") {
        file_utils::decrypt_all_files().unwrap();
    }
}
