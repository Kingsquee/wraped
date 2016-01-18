use std::path::Path;
use std::process::Command;
use super::EditorTrait;

pub struct Emacs {
    file_path: String,
    args: Vec<String>
}

impl Emacs {
    pub fn new() -> Emacs {
        Emacs {
            file_path: String::new(),
            args: Vec::new(),
        }
    }
}

impl EditorTrait for Emacs {
    fn cursor(&mut self, row:u64, col:u64) {
        self.args.push(format!("+{}:{}", row, col));
    }

    fn open(&mut self, file:&Path) {
        self.file_path = file.to_str().unwrap().to_string();
    }

    fn get_command(&self) -> Command {
        // Setup emacs server flags
        let mut command: Command = Command::new("emacsclient");
        command.arg("--alternate-editor");
        command.arg("emacs");
        
        //NOTE: Emacs arguments seem to specifically apply to the next file
        for arg in self.args.iter() {
            command.arg(arg);
        }
        
        command.arg(&self.file_path);
        
        println!("command: {:?}", command);
        command
    }
}
