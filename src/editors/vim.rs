use std::path::Path;
use std::process::Command;
use super::EditorTrait;

pub struct Vim {
    args: Vec<String>
}

impl Vim {
    pub fn new() -> Vim {
        Vim {
            args: Vec::new()
        }
    }
}

impl EditorTrait for Vim {
    fn cursor(&mut self, row:u64, col:u64) {
        // Why isn't the column jumping working?
        self.args.push("-c".to_string());
        self.args.push(format!("\"cal cursor({}, {})\"", 2, 2));
        //let s = format!("-c \"call setpos(\'.\',[0,{},{},0])\"", 4, 2);
        //println!("{}", s);
        //self.args.push(s);
    }

    fn open(&mut self, file:&Path) {
        self.args.push(format!("{}", file.to_str().unwrap()));
    }

    fn get_command(&self) -> Command {
        let mut command: Command = Command::new("gvim");
        for arg in self.args.iter() {
            command.arg(arg);
        }
        println!("command: {:?}", command);
        command
    }
}
