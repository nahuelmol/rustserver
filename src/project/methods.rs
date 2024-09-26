use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::env;
use std::io;
use serde_json;

pub fn startproject(){
    let mut s = String::new();
    println!("please enter project name");
    io::stdin()
        .read_line(&mut s)
        .expect("not a correct string");

    if let Ok(path) = env::current_dir() {
        let path = path.join("project");
        if !path.exists() {
            if let Err(_) = fs::create_dir(path.clone()) {
                println!("project cannot be created");
                return;
            };
        }

        let path = path.join(s.clone().to_string());
        println!("newproject {}", s.clone());
        if let Err(err) = fs::create_dir(path.to_str().expect("error").trim()) {
            println!("some error {}", err);
        }
        ontop(s);
    } else {
        return;
    }

}

pub fn checkprojects(){
    if let Ok(path) = env::current_dir(){
        let path = path.join("project");
        if let Ok(dir) = fs::read_dir(path) {
            for entry in dir {
                let entry = entry.unwrap();
                let mut dir = true;
                let filename = entry.file_name()
                    .to_string_lossy()
                    .into_owned();
                for c in filename.chars() {
                    if c == '.' {
                        dir = false;
                    }
                };
                if dir == true {
                    println!("{}", filename);
                }
            }
        }
    };
}


fn ontop(s:String){
    if let Ok(path) = env::current_dir() {
        let path = path.join("project/current.json");
        println!("path {:?}", path);
        let data = serde_json::json!({
            "nombre":s,
            "version":"1.0",
        });
        if !path.exists() {
            if let Err(_) = File::create(path.clone()) {
                println!("error creating file");
            }
        }
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path)
            .unwrap();

        serde_json::to_writer(file,&data).unwrap();
    }
    /*if let Ok(path) = fs::current_dir(path) {
        let path = path.join("project/current.txt");
        if path.exsits() {
            if let Ok(file) = File::open(path){
                
            };
        };
    }*/
}

pub fn switch_projects(target:&str){
    if let Ok(path) = env::current_dir() {
        let path = path.join("project/current.json");
        let data = serde_json::json!({
            "nombre":target,
            "version":"1.0",
        });
        if !path.exists() {
            if let Err(_) = File::create(path.clone()) {
                println!("error creating file");
            }
        }
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path)
            .unwrap();

        serde_json::to_writer(file,&data).unwrap();
    }
}
