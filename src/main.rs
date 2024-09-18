use std::fs::{ File, metadata };
use std::io::{ Read, Seek, SeekFrom, stdin};
use image::{Rgb, RgbImage};
use byteorder::{ BigEndian, ByteOrder };

use std::env;
use std::path::PathBuf;
use giga_segy_in::SegyFile;

mod cmds;
mod objs;

use objs::obj::CliCommand;
use cmds::cmd_switch::switcher;

fn main() {
    let mut myargs: Vec<String> = Vec::new();
    for arg in env::args() {
        myargs.push(arg);
    };
    let command = CliCommand::new(myargs); 
    if !command.is_valid() {
        println!("the command is not valid");
    } else {
        switcher(&command);
        println!("the command is valid");
    }

    let path = "/home/molinahuel/rust_projects/rustserver/data";
    let dir = PathBuf::from(path);
    let full_path = dir.join("Line_301.segy");
    let _file = SegyFile::open(full_path.to_str().unwrap(), Default::default()).unwrap();

    /*let text_header: &str = file.get_text_header();
    println!("Text header: {:?}", text_header);

    for line in file.get_text_header_lines() {
        println!("{}", line);
    }

    let bin_header = file.get_bin_header();
    println!("Bin header: {}", bin_header);*/

    /*for trace in file.traces_iter() {
        println!("Trace header: {}", trace.get_header());
        let data:Vec<f32> = file.get_trace_data_as_f32_from_trace(trace).unwrap();
        println!("Data: {:?}", data);
    }*/
}
