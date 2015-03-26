#![feature(old_io)]
#![feature(old_path)]
use std::old_path::{Path, GenericPath};
use std::old_io::Command;
use super::EditorTrait;

pub struct Emacs {
    args: Vec<String>
}

impl Emacs {
    pub fn new() -> Emacs {
        Emacs {
            args: Vec::new()
        }
    }
}

impl EditorTrait for Emacs {
    fn cursor(&mut self, row:u64, col:u64) {
        self.args.push(format!("+{}:{}", row, col));
    }

    fn open(&mut self, file:&Path) {
        self.args.push(format!("--file"));
        self.args.push(format!("{}", file.as_str().unwrap()));
    }

    fn get_command(&self) -> Command {
        let mut command: Command = Command::new("emacs");
        for arg in self.args.iter() {
            command.arg(arg);
        }
        //println!("command: {:?}", command);
        command
    }
}
