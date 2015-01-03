use std::io::Command;
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
    fn cursor(&mut self, row:uint, col:uint) {
        self.args.push(format!("--line"));
        self.args.push(format!("{}", row));
        self.args.push(format!("--column"));
        self.args.push(format!("{}", col));
    }

    fn open(&mut self, file:&Path) {
        self.args.push(format!("{}", file.as_str().unwrap()));
    }

    fn get_command(&self) -> Command {
        let mut command: Command = Command::new("kate");
        for arg in self.args.iter() {
            command.arg(arg);
        }
        println!("command: {}", command);
        command
    }
}
