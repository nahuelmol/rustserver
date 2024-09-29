use std::io::stdin;

pub fn display_trace(){
    println!("please, give me the trace number?");
    let mut opc = String::new();
    stdin()
        .read_line(&mut opc)
        .expect("err at stdin");
    println!("trace is {}", opc);
}
