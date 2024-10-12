use std::io::stdin;
use std::env;
use gnuplot::{Figure, Caption, Color};
use gnuplot::{ AxesCommon, Graph};
use giga_segy_in::SegyFile;

pub fn get_trace_data(opc:String) -> Result<Vec<f32>,String> {
    let opt:i32 = opc.trim().parse().unwrap(); 
    if let Ok(path) = env::current_dir() {
        let path = path.join("data")
            .join("seismic.segy");
        let file = SegyFile::open(path.to_str().unwrap(), Default::default()).unwrap();
        let mut i = 0;
        for trace in file.traces_iter() {
            if i == opt { 
                let data:Vec<f32> = file.get_trace_data_as_f32_from_trace(trace).unwrap();
                return Ok(data);
            };
            i+=1;
        }
    } else {
        return Err("error reading the file".to_string());
    };
    return Err("hello".to_string());
}

fn graph_trace(mut fg:Figure, data:Vec<f32>, xaxes:Vec<i32>) {
    fg.axes2d()
        .set_title("A plot", &[])
        .set_legend(Graph(0.5), Graph(0.9), &[], &[])
        .set_x_label("x", &[])
        .lines(
            &data,
            &xaxes,
            &[Color("red")],
        );
    fg.show().unwrap();
}

pub fn display_trace(){
    println!("please, give me the trace number?");
    let mut opc = String::new();
    stdin()
        .read_line(&mut opc)
        .expect("err at stdin");

    let samples = match get_trace_data(opc) {
        Ok(data) => data,
        Err(_)=> {
            println!("getting out");
            return;
        },
    };

    let len:i32 = samples.len().try_into().unwrap();
    let xaxes: Vec<i32> = (0..len).collect();
    let fg = Figure::new();
    graph_trace(fg, samples, xaxes);
}
