use crate::filesys::file::currentp_name;
use crate::objs::obj::CliCommand;
use crate::displays::traces::get_trace_data;

use std::fs::{ File, remove_file };
use std::env;
use std::io::{ BufReader, BufWriter, BufRead, Write };
use std::fs;
use std::fs::OpenOptions;

use giga_segy_in::SegyFile;
use gnuplot::{ Graph, AxesCommon, Figure, Color };

fn create_display() {
    match fs::create_dir("display") {
        Ok(_) => println!("dir created"),
        Err(err) => println!("err creating dir: {}", err),
    }
}

fn data_script() {
    if let Ok(datapath) = env::current_dir() {
        let datapath = datapath.join("display")
            .join("data.dat");
        if datapath.exists() {
            match remove_file(datapath.clone()) {
                Ok(_) => println!("{} removed", datapath.display()),
                Err(err) => println!("err {}", err),
            }
        }
        
        create_display();
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(datapath)
            .expect("creating file");
        let mut writer = BufWriter::new(file);
        let mut samples:i32 = 10;
        let mut traces:i32 = 10;

        for i in 1..samples {
            let mut line = String::new();
            for j in 1..traces {
                line = [line, format!("{}", j)].concat();
            }
            writeln!(writer, "{}", line)
                .expect("writeln error");
        }
    }
}

fn gnu_script() {
    let content = r#"
set title "seccion"
set xlabel "offset"
set ylabel "time"
    "#;
    if let Ok(path) = env::current_dir() {
        let gnupath = path.join("display")
            .join("index.gnuplot");
        if gnupath.exists() {
            match fs::remove_file(gnupath.clone()) {
                Ok(_) => println!("removing {}", gnupath.display()),
                Err(err) => println!("error removing file: {}", err),
            };
        }
        create_display();
        match File::create(gnupath) {
            Ok(mut file) => {
                file.write_all(content.as_bytes())
                .expect("It cannot be written");
            },
            Err(_) => println!("err"),
        }
    }
}

fn display(projectname:String, target:&str) {
    if let Ok(path) = env::current_dir() {
        let path = path.join("project")
            .join(projectname.trim())
            .join(target);

        let file = SegyFile::open(path.to_str().unwrap(), Default::default()).unwrap();
        let ntraces = file.get_bin_header().no_traces;
        let mut fg = Figure::new();
        fg.set_multiplot_layout(1,ntraces.into());
        for i in 0..ntraces {
            let samples = match get_trace_data(i.to_string()) {
                Ok(data) => data,
                Err(_)=> {
                    println!("getting out");
                    return;
                },
            };
            let len:i32 = samples.len().try_into().unwrap();
            let xaxes: Vec<i32> = (0..len).collect();
            fg.axes2d()
                .set_title("A plot", &[])
                .set_legend(Graph(0.5), Graph(0.9), &[], &[])
                .set_x_label("x", &[])
                .lines(
                    &samples,
                    &xaxes,
                    &[Color("red")],
                );
        }
        fg.show().unwrap();
    };
}

pub fn display_image(target:&str, cmd:&CliCommand) {
    let projectname = match currentp_name() {
        Ok(name) => name,
        Err(_) => { 
            println!("error getting the name");
            return;
        },
    };

    if let Ok(path) = env::current_dir() {
        let state = match cmd.get_state() {
            Ok(state) => state,
            Err(_) => {
                return;
            },
        };
        let path = path.join("project")
            .join(projectname.trim())
            .join("register.txt");
        let file = match File::open(path) {
            Ok(file) => file,
            Err(_) => {
                return;
            },
        };
        let command = [target," "].concat();
        let command = [command, state.to_string()].concat();
        let lector = BufReader::new(file);
        let mut exists = 0;
        for line in lector.lines() {
            match line {
                Ok(line) => {
                    if line == command {
                        exists+=1;
                    } 
                },
                Err(_) => {
                    return;
                },
            }
        };
        /*if exists > 0 {
            println!("file exists");
            display(projectname, target);
        } else {
            println!("file doesnt exists");
        }*/

        gnu_script();
        //data_script();
    };

}
