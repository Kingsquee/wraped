use std::path::Path;
use std::process::Command;
use super::EditorTrait;

pub struct Kate {
    args: Vec<String>,
}

impl Kate {
    pub fn new() -> Kate {
        Kate {
            args: Vec::new()
        }
    }
}

impl EditorTrait for Kate {
    fn cursor(&mut self, row:u64, col:u64) {
        self.args.push(format!("--line"));
        self.args.push(format!("{}", row));
        self.args.push(format!("--column"));
        self.args.push(format!("{}", col));
    }

    fn open(&mut self, file:&Path) {
        self.args.push(format!("{}", file.to_str().unwrap()));
    }

    fn get_command(&self) -> Command {
        let mut command: Command = Command::new("kate");
        for arg in self.args.iter() {
            command.arg(arg);
        }
        //println!("command: {:?}", command);
        command
    }
}
