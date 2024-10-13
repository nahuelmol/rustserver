use crate::filesys::file::currentp_name;
use crate::objs::obj::CliCommand;
use crate::displays::traces::get_trace_data;

use std::fs::{ File, remove_file };
use std::env;
use std::io::{ BufReader, BufWriter, BufRead, Write };
use std::fs;
use std::fs::OpenOptions;
use std::path::PathBuf;

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


fn datas_builder(path:PathBuf, trace) {
    //this save the data from data.dat into the data arrays in gnuplot
    let file = OpenOptions::new()
        .create(false)
        .write(true)
        .open(path)
        .expect("opening file");
    let mut writer = BufWriter::new(file);
    let nsamples = trace.len();
    let mut samples:i32 = 10;
    let mut traces:i32 = 10;

    let forline = format!("for ({}:{}) {", 1, ntraces);
    writeln!(writer, "{}", forline);

    let arrayline = format!("array data{}[{}] \n}", ntraces, nsamples);
    writeln!(writer, "{}", arrayline);

    let forline = format!("for ({}:{}) {", 1, ntraces);
    writeln!(writer, "{}", forline);

    for i in ntraces {
        let mut line = String::new();
        let trace = get_trace_data(i.to_string(), target.to_string());
        for j in nsamples {
            
            trace[j];
        }
    }

    for sample in trace.iter() {
        let mut line = String::new();
        for j in 1..traces {
            line = [line, format!("{}", j)].concat();
        }
        writeln!(writer, "{}", line)
            .expect("writeln error");
    }
}

fn gnu_script(target:&str) {
    let content = r#"
set title "seccion"
set xlabel "offset"
set ylabel "time"

offset_inicio=1
tiempo_inicio=1
    "#;
    if let Ok(path) = env::current_dir() {
        let path = path.join("data")
            .join(target);

        let file = SegyFile::open(path
                .to_str().unwrap(), 
                Default::default())
                .unwrap();
        let bin_header = file.get_bin_header();
        let num_traces = bin_header.no_traces;
        let num_sampls = bin_header.no_samples;

        let offset_fin = format!("offset_fin={}\n", num_traces);
        let tiempo_fin = format!("tiempo_fin={}\n", num_sampls);
        let content = [content,&offset_fin].concat();
        let content = [content,tiempo_fin].concat();

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
            match File::create(gnupath.clone()) {
                Ok(mut file) => {
                    file.write_all(content.as_bytes())
                    .expect("It cannot be written");
                },
                Err(_) => println!("err"),
            }
            datas_builder(gnupath, trace);
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
            let samples = match get_trace_data(i.to_string(), target.to_string()) {
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

        gnu_script(target);
        //data_script();
    };

}
