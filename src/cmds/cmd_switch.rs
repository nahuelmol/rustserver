
use crate::objs::obj::CliCommand;
use crate::cmds::filters::filter_switch;

pub fn switcher(cmd:&CliCommand){
    let action = cmd.get_action();
    if action == "read" {
        println!("reading");
        let _target = cmd.get_target();
    } else if action == "filt" {
        let filter = cmd.get_filter_type();
        filter_switch(filter, cmd.get_target());
    } else {
        println!("not registered");
    }
}
 

