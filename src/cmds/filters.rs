
pub fn passband(_target:&str){
    println!("passband");
}

pub fn filter_switch(filter_type:&str, target:&str){
    if filter_type == "pb" {
        passband(target);
    } else if filter_type == "hp" {
        println!("doing {}", filter_type);
    } else if filter_type == "lp" {
        println!("doing {}", filter_type);
    } else {
        println!("fll with something");
    }
}
