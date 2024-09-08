use std::fs::File;
use std::io::{ Read, Seek, SeekFrom};
use plotters::prelude::*;

fn segreader(file: &mut File){
    let mut header = [0;3600];
    if let Err(_) = file.read_exact(&mut header){
        println!("err!");
        return;
    }

    if let Ok(meta) = file.metadata() {
        println!("file size -> {}",meta.len());
    }

    let ns = u16::from_be_bytes([header[3221],header[3222]]);
    let dt = u16::from_be_bytes([header[3225],header[3226]]);
    let fmt = u16::from_be_bytes([header[3227],header[3227]]);

    println!("samples per trace: {}", ns);
    let fmt_size = match fmt {
        1 => 4,
        2 => 4,
        3 => 2,
        4 => 4,
        5 => 4,
        6 => 2,
        0 => { 
            println!("unsupported format");
            4
        },
        7 => 2,
        8 => 2,
        9 => 2,
        10_u16..=u16::MAX => todo!(),
    };
    let trace_size = ns as usize * (fmt_size + 1);
    if let Err(_) = file.seek(SeekFrom::Start(3600)){
        return;
    };
    let mut trace = vec![0;trace_size];
    if let Err(_) = file.read_exact(&mut trace){
        return;
    }
    let samples: Vec<i16> = trace
        .chunks_exact(2)
        .map(|bytes| i16::from_be_bytes([bytes[0],bytes[1]]))
        .collect();

    let root = BitMapBackend::new("plotters-doc-data/0.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)
        .unwrap();

    chart.configure_mesh().draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
            &RED,
        ))
        //.label("y = x^2")
        //.legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED))
        .unwrap();
    

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .unwrap();

    root.present()
        .unwrap();
}

fn main() {
    let filename = "data/seismic.segy";
    match File::open(filename) {
        Ok(mut file) => segreader(&mut file),
        Err(_) => println!("not readable"),
    }
}
