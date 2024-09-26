use std::fs;
use std::env;
use serde_json::Value;
use std::fs::File;

use std::io::Read;

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

pub fn current_project() {
    if let Ok(path) = env::current_dir() {
        let path = path.join("project/current.json");
        if let Ok(mut file) = File::open(path) {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            let jsoned: Value = serde_json::from_str(&content).unwrap();
            println!("{}", jsoned["nombre"].as_str().unwrap());
        };
    } else {
        return;
    };

}
