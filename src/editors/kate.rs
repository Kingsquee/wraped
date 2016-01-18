use std::path::Path;
use std::process::Command;
use super::EditorTrait;

pub struct Kate {
    file_path: String,
    args: Vec<String>,
}

impl Kate {
    pub fn new() -> Kate {
        Kate {
            file_path: String::new(),
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
        self.file_path = file.to_str().unwrap().to_string();
    }

    fn get_command(&self) -> Command {
        let mut command = Command::new("kate");

        command.arg(&self.file_path);

        for arg in self.args.iter() {
            command.arg(arg);
        }

        println!("command: {:?}", command);
        command
    }
}
