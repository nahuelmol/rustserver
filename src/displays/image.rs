use crate::filesys::file::currentp_name;
use crate::objs::obj::CliCommand;
use crate::displays::traces::get_trace_data;

use std::fs::{ File, remove_file };
use std::env;
use std::io::{ BufReader, BufWriter, BufRead, Write, Seek, SeekFrom };
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

fn data_script() -> Result<Vec<Vec<f32>>, String>{
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
        let samples:i32 = 10;
        let traces:i32 = 4;

        let ntraces = traces as usize;
        let mut matrix: Vec<Vec<f32>> = Vec::new();
        for j in 0..ntraces {
            let trace = match get_trace_data(j.to_string(),"seismic.segy".to_string()) {
                Ok(trace) => trace,
                Err(_) => return Err("error".to_string()),
            };
            let tracelen = trace.len() - 1;
            println!("{} trace", j);
            println!("{} tracelen", tracelen);
            println!("{} sample", trace[0]);
            for (i, elem) in trace.iter().enumerate() {
                if j == 0 {
                    matrix.push(vec![*elem]);
                }
            }
            for (i,_) in trace.iter().enumerate() {
                if j > 0 {
                    matrix[i].push(trace[i]);
                }
            }
        }
        for j in matrix.iter() {
            let mut line = String::new();
            for elem in j.iter() {
                line = [line, format!("{}\t", elem.to_string())].concat();
            }
            writeln!(writer, "{}", line)
                .expect("writeln error");
        }
        return Ok(matrix);
    } else {
        return Err("error, path not found".to_string());
    }
}


fn datas_builder(path:PathBuf, nsamples:usize, ntraces:usize) {
    //this save the data from data.dat into the data arrays in gnuplot
    let mut file = OpenOptions::new()
        .create(false)
        .write(true)
        .open(path)
        .expect("opening file");
    file.seek(SeekFrom::End(0)).
        expect("no se pudo buscar la ultima linea");
    let mut writer = BufWriter::new(file);

    writeln!(writer, "\n");
    for n in 0..ntraces {
        let arrayline = format!("array data{}[{}]", n, nsamples);
        writeln!(writer, "{}", arrayline);
    }

    writeln!(writer, "unset key");
    writeln!(writer, "\n");

    for n in 0..ntraces {
        let head= format!("do for [i=1:{}] {{", nsamples);
        let arg = format!("\"awk 'NR==\"{} \" 'data.dat'",n);
        let cnt = format!("\tdata{}[i] = real(word(system({}),{}))",n,arg,n);
        let end = "})".to_string();
        writeln!(writer,"{}", head);
        writeln!(writer,"{}", cnt);
        writeln!(writer,"{}", end);
    }

    writeln!(writer, "\n");
    writeln!(writer, "my_data(x) = data[x]");
    writeln!(writer, "set parametric");
    let plotfor = format!("plot for [offset=offset_inicio:offset_fin-1] my_data(t) + (offset-offset_inicio) * intervalo, \
        \n\tt with lines");
    writeln!(writer, "{}", plotfor);
}

fn gnu_script(target:&str, nsamples:usize, ntraces:usize) {
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
        let _bin_header = file.get_bin_header();

        let offset_fin = format!("offset_fin={}\n", ntraces);
        let tiempo_fin = format!("tiempo_fin={}\n", nsamples);
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
            match File::create(gnupath.clone()) {
                Ok(mut file) => {
                    file.write_all(content.as_bytes())
                    .expect("It cannot be written");
                },
                Err(_) => println!("err"),
            }
            datas_builder(gnupath, nsamples, ntraces);
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

        let data = match data_script() {
            Ok(data) => data,
            Err(_) => return,
        };
        gnu_script(target,data.len(), data[0].len());
    };

}
