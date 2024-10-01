

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

    pub fn is_target(&self) -> bool {
        let len = self.flags.len() as i32;
        if len > 1 {
            true
        } else {
            false
        }
    } 

    pub fn get_filter_type(&self) -> &str {
        &self.flags[2]
    }

    pub fn get_flags_size(&self) -> usize {
        self.flags.len()
    }

    pub fn get_state(&self) -> Result<&str, &str> {
        let result = std::panic::catch_unwind(||&self.flags[2]);
        match result {
            Ok(opc) => {
                return Ok(opc);
            },
            Err(_)=> {
                return Err("error");
            }

        }
    }
}
