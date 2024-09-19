
pub fn passband(_target:&str){
    println!("passband");
}

pub fn highpass(){}
pub fn lowpass(){}

pub fn filter_switch(filter_type:&str, target:&str){
    if filter_type == "pb" {
        passband(target);
    } else if filter_type == "hp" {
        highpass();
        println!("doing {}", filter_type);
    } else if filter_type == "lp" {
        lowpass();
        println!("doing {}", filter_type);
    } else {
        println!("fll with something");
    }
}
