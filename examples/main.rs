extern crate wraped;
extern crate getopts;

use std::os;
use std::io::fs::File;
use std::io::process::StdioContainer;
use getopts::{optopt,getopts};
use wraped::{Editor, EditorTrait};

fn main() {
    // Program args

    let args: Vec<String> = os::args();
    let opts = &[
        optopt("e", "editor", "Open the state struct in the editor of choice.", "EDITOR")
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let editor_string = match matches.opt_str("e") {
        Some(string) => { string }
        None => { panic!("Please use '--editor youreditorhere' to launch an editor.") }
    };

    let mut editor = match Editor::new(editor_string.as_slice()) {
        Some(e) => e,
        None => panic!("Sorry, that editor isn't supported."),
    };

    let file_path = Path::new("test.txt");
    let mut file = File::create(&file_path);
    file.write_str(
"Oh boy this is a file alright.
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
    command.stdout(StdioContainer::InheritFd(1));
    command.stderr(StdioContainer::InheritFd(2));
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
