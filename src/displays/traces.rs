use std::io::stdin;
use std::env;
use gnuplot::{Figure, Caption, Color};
use gnuplot::{ AxesCommon, Graph};
use giga_segy_in::SegyFile;
//use std::str::FromStr;
//use std::process::Command;
//use std::io::Write;

fn get_trace_data(opc:String) -> Result<Vec<f32>,String> {
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
    let mut fg = Figure::new();
    let mut sample2: Vec<f32> = Vec::new();
    for valor in samples.clone() {
        sample2.push(valor + (0.2 as f32));
    }
    fg.set_multiplot_layout(1,10);

    fg.axes2d()
        .set_title("A plot", &[])
        .set_legend(Graph(0.5), Graph(0.9), &[], &[])
        .set_x_label("x", &[])
        .set_y_label("y^2", &[])
        .lines(
            &samples,
            &xaxes,
            &[Color("blue"), Caption("hello")],
        );
    fg.axes2d()
        .set_title("A plot", &[])
        .set_legend(Graph(0.5), Graph(0.9), &[], &[])
        .set_x_label("x", &[])
        .lines(
            &sample2,
            &xaxes,
            &[Color("red")],
        );
    for i in 0..8 {
        fg.axes2d()
            .set_title("A plot", &[])
            .set_x_label("x", &[])
            .lines(
                &sample2,
                &xaxes,
                &[Color("red")],
            );
    }
    fg.show().unwrap();
    let _ = fg.save_to_png("example.png",800,600);
}
