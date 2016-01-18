use std::path::Path;
use std::process::Command;
use super::EditorTrait;

pub struct Vim {
    file_path: String,
    args: Vec<String>,
}

impl Vim {
    pub fn new() -> Vim {
        Vim {
            file_path: String::new(),
            args: Vec::new()
        }
    }
}

// To avoid opening new instances, we have to tie into Vim's client/server model.
// We also have to write the arguments as vim commands, due to the nature of interaction with
// vim servers

impl EditorTrait for Vim {
    fn cursor(&mut self, row:u64, col:u64) {
        self.args.push(format!("call cursor({}, {})", row, col));
    }

    fn open(&mut self, file:&Path) {
        self.file_path = file.to_str().unwrap().to_string();
    }

    fn get_command(&self) -> Command {
        use std::process::Output;
        
        // Access the server instance
        let server_list: Output = Command::new("gvim").arg("--serverlist").output().unwrap();
        println!("Server list: {}", String::from_utf8_lossy(&server_list.stdout));
        
        let last_used_server_name = String::from_utf8_lossy(&server_list.stdout).split_whitespace().last().unwrap_or("GVIM").to_string();
        println!("Server name: {}", last_used_server_name);
        
        let mut command: Command = Command::new("gvim");
        
        command.arg("--servername".to_string());
        command.arg(last_used_server_name);
        command.arg("--remote".to_string());

        let mut args = "+".to_string();

        for i in 0..self.args.len() {
            args.push_str(&self.args[i]);
            if i != self.args.len()-1 {
                args.push('|');
            }
        }
        command.arg(args);

        command.arg(&self.file_path);

        println!("command: {:?}", command);
        command
    }
}
