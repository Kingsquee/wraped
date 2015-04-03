extern crate wraped;
extern crate getopts;

use std::env;
use std::path::{Path};
use std::io::Write;
use std::fs::File;
use std::process::Stdio;
use getopts::Options;
use wraped::{Editor, EditorTrait};

fn main() {
    // Program args

    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("e", "editor", "Open the state struct in the editor of choice.", "EDITOR");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let editor_string = match matches.opt_str("e") {
        Some(string) => { string }
        None => { panic!("Please use '--editor youreditorhere' to launch an editor.") }
    };

    let mut editor = match Editor::new(&editor_string) {
        Some(e) => e,
        None => panic!("Sorry, that editor isn't supported."),
    };

    let file_path = Path::new("test.txt");
    let mut file = File::create(&file_path).unwrap();
    file.write_all(
b"Oh boy this is a file alright.
Sure is.
Hey buddy sign your name here:
Thanks a lot it means a lot to me :))))
cya bby"
).unwrap();
    file.flush().unwrap();

    editor.cursor(3, 32);
    editor.open(&file_path);
    let mut command = editor.get_command();

    // Try to run this thing
    command.stdout(Stdio::inherit());
    command.stderr(Stdio::inherit());
    let mut result = match command.spawn() {
        Ok(r) => r,
        Err(e) => panic!("Failed to run: {}", e),
    };

    // If it ran, how'd it do?

    if !result.wait().unwrap().success() {
        panic!("Build failed");
    };

}

pub fn test() {
    let mut e = Editor::new("kate").unwrap();
    e.cursor(3,4);
}
