use std::fs;
use std::fs::metadata;
use std::fs::File;
use std::io::Error;
use std::sync::mpsc;
use std::thread;

fn create_tar_gz(folder_name: &str) {
    let mut fname = folder_name.to_string().clone();
    fname.push_str(".tar.gz");
    let tar_gz = File::create(&fname).unwrap();
    let mut tar = tar::Builder::new(tar_gz);
    tar.append_dir_all(folder_name, folder_name).unwrap();
    println!("Tarring {} to {}", folder_name, &fname);
}

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
