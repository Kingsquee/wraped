use std::path::Path;
use std::process::Command;
use super::EditorTrait;

pub struct Gedit {
    args: Vec<String>,
}

impl Gedit {
    pub fn new() -> Gedit {
        Gedit {
            args: Vec::new()
        }
    }
}

impl EditorTrait for Gedit {
    fn cursor(&mut self, row:u64, col:u64) {
        // gedit 3.18.1 seems to treat columns differently? 
        self.args.push(format!("+{}:{}", row, if col == 0 { 0 } else { col - 1 }));
    }

    fn open(&mut self, file:&Path) {
        self.args.push(format!("{}", file.to_str().unwrap()));
    }

    fn get_command(&self) -> Command {
        let mut command: Command = Command::new("gedit");
        for arg in self.args.iter() {
            command.arg(arg);
        }
        println!("command: {:?}", command);
        command
    }
}
