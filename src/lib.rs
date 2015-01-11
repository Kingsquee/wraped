use editors::{Kate, Emacs};
use std::ascii::AsciiExt;
use std::io::Command;
pub use editors::EditorTrait;

pub mod editors;

impl Editor {
    pub fn new(name: &str) -> Option<Editor> {
        match name.to_ascii_lowercase().as_slice() {
            "kate" => Some(Editor::Kate(Box::new(Kate::new()))),
            "emacs" => Some(Editor::Emacs(Box::new(Emacs::new()))),
            _ => None,
        }
    }
}

pub enum Editor {
    Kate (Box<Kate>),
    Emacs (Box<Emacs>),
}

impl EditorTrait for Editor {
    fn cursor(&mut self, row:u64, col:u64) {
        match self {
            &mut Editor::Kate(ref mut e) => e.cursor(row, col),
            &mut Editor::Emacs(ref mut e) => e.cursor(row, col),
        }
    }

    fn open(&mut self, file:&Path) {
        match self {
            &mut Editor::Kate(ref mut e) => e.open(file),
            &mut Editor::Emacs(ref mut e) => e.open(file),
        }
    }

    fn get_command(&self) -> Command {
        match self {
            &Editor::Kate(ref e) => e.get_command(),
            &Editor::Emacs(ref e) => e.get_command(),
        }
    }
}
