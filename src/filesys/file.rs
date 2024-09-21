use std::env;
use std::fs::{ File, OpenOptions };
use std::io::{ Write, Read };
use serde_json::Value;

use giga_segy_in::SegyFile;

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

pub fn loadfile(target:&str){
    if let Ok(mainpath) = env::current_dir() {
        let path = mainpath.join("project").join("current.json");
        let mut file = File::open(path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let json: Value = serde_json::from_str(&content)
            .unwrap();
        let projectname = json["nombre"].as_str()
            .unwrap();

        let newline = [target, " up"].concat();
        let registerpath = mainpath.join("project")
            .join(projectname.trim())
            .join("register.txt");

        if !registerpath.exists() {
            println!("path {:?}",registerpath);
            if let Err(_) = File::create(registerpath.clone()) {
                println!("cannot create file");
            };
        }
        /*if let Ok(mut file) = File::open(registerpath) {
            file.write_all(newline)
                .unwrap();
        }*/

        let mut file = OpenOptions::new()
            .append(true)
            .open(registerpath)
            .unwrap();

        writeln!(file, "{}", newline.as_str()).unwrap();
    };

}
