use std::fs;
use std::env;
use std::io;

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
        let path = path.join(s);

        if let Err(err) = fs::create_dir(path) {
            println!("some error {}", err);
        }
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
                println!("{}", entry.file_name()
                    .to_string_lossy()
                    .into_owned());
            }
        }
    };
}
