use std::path::{Path};
use editors::{Kate, Gedit, Emacs, Vim};
use std::ascii::AsciiExt;
use std::process::Command;
pub use editors::EditorTrait;

pub mod editors;

impl Editor {
    pub fn new(name: &str) -> Option<Editor> {
        match name.to_ascii_lowercase().as_ref() {
            "kate" => Some(Editor::Kate(Box::new(Kate::new()))),
            "gedit" => Some(Editor::Gedit(Box::new(Gedit::new()))),
            "emacs" => Some(Editor::Emacs(Box::new(Emacs::new()))),
            "vim" => Some(Editor::Vim(Box::new(Vim::new()))),
            _ => None,
        }
    }
}

pub enum Editor {
    Kate (Box<Kate>),
    Gedit (Box<Gedit>),
    Emacs (Box<Emacs>),
    Vim (Box<Vim>),
}

impl EditorTrait for Editor {
    fn cursor(&mut self, row:u64, col:u64) {
        match self {
            &mut Editor::Kate(ref mut e) => e.cursor(row, col),
            &mut Editor::Gedit(ref mut e) => e.cursor(row, col),
            &mut Editor::Emacs(ref mut e) => e.cursor(row, col),
            &mut Editor::Vim(ref mut e) => e.cursor(row, col),
        }
    }

    fn open(&mut self, file: &Path) {
        match self {
            &mut Editor::Kate(ref mut e) => e.open(file),
            &mut Editor::Gedit(ref mut e) => e.open(file),
            &mut Editor::Emacs(ref mut e) => e.open(file),
            &mut Editor::Vim(ref mut e) => e.open(file),
        }
    }

    fn get_command(&self) -> Command {
        match self {
            &Editor::Kate(ref e) => e.get_command(),
            &Editor::Gedit(ref e) => e.get_command(),
            &Editor::Emacs(ref e) => e.get_command(),
            &Editor::Vim(ref e) => e.get_command(),
        }
    }
}
