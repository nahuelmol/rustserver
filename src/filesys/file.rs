use std::io;
use std::fs;
use std::env;
use std::path::Path;

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
            let text_header: &str = file.get_text_header();
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

pub fn loadfile(target:&str){}
