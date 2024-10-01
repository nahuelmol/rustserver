use crate::filesys::file::currentp_name;
use crate::objs::obj::CliCommand;
use std::fs::File;
use std::env;
use std::io::BufReader;
use std::io::BufRead;

pub fn display_image(target:&str, cmd:&CliCommand) {
    let projectname = match currentp_name() {
        Ok(name) => name,
        Err(_) => { 
            println!("error getting the name");
            return;
        },
    };

    if let Ok(path) = env::current_dir() {
        let state = match cmd.get_state() {
            Ok(state) => state,
            Err(_) => {
                return;
            },
        };
        let path = path.join("project")
            .join(projectname.trim())
            .join("register.txt");
        let file = match File::open(path) {
            Ok(file) => file,
            Err(_) => {
                return;
            },
        };
        let command = [target," "].concat();
        let command = [command, state.to_string()].concat();
        let lector = BufReader::new(file);
        let mut exists = 0;
        for line in lector.lines() {
            match line {
                Ok(line) => {
                    if line == command {
                        exists+=1;
                    } 
                },
                Err(_) => {
                    return;
                },
            }
        };
        if exists > 0 {
            println!("file exists");
        } else {
            println!("file doesnt exists");
        }
    };

}
