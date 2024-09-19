
pub fn startproject(){

    let must s = String::new();
    println!("please enter the name");
    stdin()
        .read_line(&mut s)
        .expect("not a correct string");
    println!("start {} project", s);
}

pub fn checkprojects(){
    println!("checking projects");
}
