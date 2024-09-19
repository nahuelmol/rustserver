

pub fn switcher(cmd:str, body){

    if cmd.action == "read" {
        println!("reading");
        let _filename = body[0];
    } else {
        println!("not registered");
    }
}
 

