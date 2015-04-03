use std::path::Path;
use std::process::Command;
pub use self::kate::Kate;
pub use self::emacs::Emacs;
mod kate;
mod emacs;

pub trait EditorTrait {
    fn cursor(&mut self, row: u64, col: u64);
    fn open(&mut self, file: &Path);
    fn get_command(&self) -> Command;
}
