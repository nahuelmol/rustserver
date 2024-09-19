use std::io;
use std::fs;
use std::env;
use std::path::Path;


pub fn check_raw_target(target:&str){
    if let Ok(path) =  env::current_dir() {
        let path = path.join("data/");
        let path = path.join(target);
        if path.exists() {
            println!("file exists");
        } else {
            println!("file not exists");
        }
    }
}

pub fn get_all_content(){
    if let Ok(path) =  env::current_dir() {
        let datapath = path.join("data");
        if let Ok(dir) = fs::read_dir(datapath) {
            for entry in dir {
                let entry = entry.unwrap();
                println!("{}", entry.file_name().to_str().unwrap());
            }
        }
    }
}
