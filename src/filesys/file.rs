use std::env;
use std::fs::{ File, OpenOptions };
use std::io::{ Write, Read, BufReader, BufRead };
use std::fs;
use serde_json::Value;

use giga_segy_in::SegyFile;
use crate::objs::obj::CliCommand;

pub fn readfile(target:&str) {
    if let Ok(path) =  env::current_dir() {
        if path.exists() {
            let path = path.join("data").join(target);
            println!("reading {}", path.display());
            let file = SegyFile::open(path
                    .to_str().unwrap(), 
                    Default::default())
                .unwrap();
            let _text_header: &str = file.get_text_header();
            let bin_header = file.get_bin_header();
            let num_traces = bin_header.no_traces;
            let num_sampls = bin_header.no_samples;
            let sample_interval = bin_header.sample_interval;
            let sample_fcode = bin_header.sample_format_code;
            let measure_syst = bin_header.measurement_system;
            println!("n traces {}", num_traces);
            println!("n sampls {}", num_sampls);
            println!("sample interval {}", sample_interval);
            println!("sample format code {}", sample_fcode);
            println!("measurement system {:?}", measure_syst);
        } else {
            println!("file not exists");
        }
    }
}

pub fn currentp_name() -> Result<String, String> {
    if let Ok(path) = env::current_dir() {
        let pathproject = path.join("project")
            .join("current.json");
        let mut file = File::open(pathproject).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let json: Value = serde_json::from_str(&content)
            .unwrap();
        let projectname = json["nombre"].as_str()
            .unwrap();
        return Ok(projectname.to_string());
    } else {
        return Err("error".to_string());
    }
}


fn move_file(file: &str) {

    if let Ok(path) = env::current_dir() {
        let rawfile = path.join("data").join(file);
        let projectname = match currentp_name() {
            Ok(name) => name,
            Err(_) => { 
                println!("error getting the name");
                return;
            },
        };
        let file_to_add = path.join("project").join(projectname.trim()).join(file);
        println!("{}", file_to_add.display());
        if !file_to_add.exists() {
            if let Err(err) = File::create(file_to_add.clone()) {
                println!("cannot create file: {}", err);
            };
        }

        let mut newfile = match OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_to_add) {
            
            Ok(file) => file,
            Err(err) => { 
                println!("error: {}", err);
                return;
            }
        };
        match fs::read(rawfile) {
            Ok(content) => {
                match newfile.write_all(&content) {
                    Ok(_) => println!("file written"),
                    Err(_) => println!("error at written"),
                }
            },
            Err(_) => {
                println!("error reading");
            }
        }
    }

}

pub fn loadfile(target:&str, cmd:&CliCommand){
    if let Ok(mainpath) = env::current_dir() {
        let path = mainpath.join("project").join("current.json");
        let mut file = File::open(path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let json: Value = serde_json::from_str(&content)
            .unwrap();
        let projectname = json["nombre"].as_str()
            .unwrap();

        let state = if let Ok(state_file) = cmd.get_state() {
            let state = [" ", state_file].concat();
            state
        } else {
            " up".to_string()
        };
        let newline = [target, &state].concat();
        let registerpath = mainpath.join("project")
            .join(projectname.trim())
            .join("register.txt");

        if !registerpath.exists() {
            println!("path {:?}",registerpath);
            if let Err(_) = File::create(registerpath.clone()) {
                println!("cannot create file");
            };
        }

        let mut file = OpenOptions::new()
            .append(true)
            .open(registerpath)
            .unwrap();

        writeln!(file, "{}", newline.as_str()).unwrap();
        move_file(target);
    };
}

fn take_name(line:String) -> Result<String, String> {
    let mut name = String::new();
    for c in line.chars() {
        if c == ' ' {
            return Ok(name);
        }
        name.push(c);
    }
    Err("error".to_string())
}

pub fn delete_file(file:&str) {
    if let Ok(path) = env::current_dir() { 
        let projectname = match currentp_name() {
            Ok(name) => name,
            Err(_) => { 
                println!("error getting the name");
                return;
            },
        };
        let path = path.join("project")
            .join(projectname.trim())
            .join("register.txt");
        let mut lines: Vec<String> = Vec::new();
        match File::open(path) {
            Ok(content) => {
                let content_buff = BufReader::new(content);
                for line in content_buff.lines() {
                    match line {
                        Ok(line) => {
                            if let Ok(name) = take_name(line.clone()) {
                                if name == file {
                                    lines.push(line.to_string());
                                }
                            };
                        },
                        Err(_) => println!("err")
                    }
                }
            },
            Err(_) => println!("err opening register"),
        };

        for entry in lines.iter() {
            println!("-> {}", entry);
        }
    };

}
