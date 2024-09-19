
use crate::objs::obj::CliCommand;
use crate::cmds::filters::filter_switch;
use crate::filesys::checker::check_raw_target;
use crate::filesys::checker::get_all_content;
use crate::filesys::file::{ readfile, loadfile };

use crate::project::methods::{ startproject, checkprojects };

pub fn switcher(cmd:&CliCommand){
    let action = cmd.get_action();
    if action == "read" {
        println!("reading");
        readfile(cmd.get_target())
    } else if action == "filt" {
        let len = cmd.get_flags_size();
        if len > 2 {
            let filter = cmd.get_filter_type();
            filter_switch(filter, cmd.get_target());
        } else {
            println!("you forgot arguments");
            return;
        }
    } else if action == "mute" {
        println!("muting..");
    } else if action == "check" {
        if cmd.is_target() {
            check_raw_target(cmd.get_target());
        } else {
            get_all_content();
        }
    } else if action == "load" {
        loadfile(cmd.get_target());
    } else if action == "startproject" {
        startproject();
    } else if action == "projects" {
        checkprojects();
    } else {
        println!("not registered");
    }
}
 

