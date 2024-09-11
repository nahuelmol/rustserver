use std::fs::File;
use std::io::{ Read, Seek, SeekFrom, stdin};
use image::{Rgb, RgbImage};


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
    let _dt = u16::from_be_bytes([header[3225],header[3226]]);
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

    let time_step = 1.0 / 1000.0;
    let time = (0..samples.len()).map(|x| x as f64 * time_step).collect::<Vec<f64>>();
    println!("samples {}", samples.len());
    println!("time {}",time.len());

    image_trace(samples, time);

}

fn image_trace(samples: Vec<i16>, time: Vec<f64>){
    let mysamp = &samples[0..800];
    let mytime = &time[0..800];


    let mut img = RgbImage::new(800,600);
    let mut t = 0;
    let escala = 200;
    let scaled: Vec<f64> = mysamp.iter()
        .map(|x| (*x as f64) / (escala as f64))
        .collect();
    /*let max = scaled.iter()
        .fold(f64::MAX, |a,&b| a.max(b));
    let min = scaled.iter()
        .fold(f64::MIN, |a,&b| a.min(b));
    let range = max - min;*/
        
    for y in scaled.iter(){
        let y = 300.0 - y;
        println!("y -> {}", y);
        img.put_pixel(t, y as u32, Rgb([255,0,0]));
        t += 1;
    }
    let _ = img.save("onda_senoidal.png");
}

fn image_working(a: u32){
    let mut img = RgbImage::new(800,600);
    let f = 0.01;
    let w = 2.0 * std::f64::consts::PI * f;
    let a = a as f64;

    for t in 0..800 {
        let y_t = (a * (w * (t as f64)).sin()) + 300.0;
        img.put_pixel(t, y_t as u32, Rgb([255,0,0]));
    }

    let _ = img.save("onda_senoidal.png");
}

fn main() {
    let filename = "data/seismic.segy";
    match File::open(filename) {
        Ok(mut file) => segreader(&mut file),
        Err(_) => println!("not readable"),
    }

    println!("please, amplitude..");
    let mut amplitude = String::new();
    stdin()
        .read_line(&mut amplitude)
        .expect("Error in amplitude");

    let amplitude: u32 = amplitude.trim()
        .parse()
        .expect("error parsing");

    println!("trying with -> {}", amplitude);

    image_working(amplitude);
}
