

pub struct CliCommand {
    action:String,
    flags:Vec<String>,
    valid:bool,
}

impl CliCommand {
    pub fn new(args:Vec<String>) -> Self {
        let mut flags:Vec<String> = Vec::new();
        let mut action:String = String::new();
        let valid:bool;
        
        if args.len() >= 1 {
            action = args[0].clone();
            for flg in &args[1..] {
                flags.push(flg.clone())
            }
            valid = true;
        } else {
            valid = false;
        }

        Self { 
            action,
            flags,     
            valid,
        }
    }

    pub fn is_valid(&self) -> &bool {
        &self.valid
    }

    pub fn get_action(&self) -> &str {
        &self.flags[0]
    }

    pub fn get_target(&self) -> &str {
        &self.flags[1]
    }

    pub fn get_filter_type(&self) -> &str {
        &self.flags[2]
    }
}
