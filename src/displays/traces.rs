use std::io::stdin;
use std::env;
use gnuplot::{Figure, Caption, Color};
use gnuplot::{ AxesCommon, Graph};
use giga_segy_in::SegyFile;

fn get_trace_data(_opc:String) -> Result<Vec<f32>,String> {
    if let Ok(path) = env::current_dir() {
        let path = path.join("data")
            .join("Line_301.segy");
        let file = SegyFile::open(path.to_str().unwrap(), Default::default()).unwrap();
        let text_header: &str = file.get_text_header();
        let mut i = 0;
        for trace in file.traces_iter() {
            if i == 0 { 
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
    println!("trace is {}", opc);

    let samples = match get_trace_data(opc) {
        Ok(data) =>  data,
        Err(_) => {
            println!("bad move");
            return;
        },
    };

    let len:i32 = samples.len().try_into().unwrap();
    let xaxes: Vec<i32> = (0..len).collect();
    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("A plot", &[])
        .set_legend(Graph(0.5), Graph(0.9), &[], &[])
        .set_x_label("x", &[])
        .set_y_label("y^2", &[])
        .lines(
            &samples,
            &xaxes,
            &[Caption("Parabola")],
        );
    fg.show().unwrap();
}
