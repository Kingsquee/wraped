use std::io::Command;
pub use self::kate::Kate;
pub use self::emacs::Emacs;
mod kate;
mod emacs;

pub trait EditorTrait {
    fn cursor(&mut self, row:uint, col:uint);
    fn open(&mut self, file:&Path);
    fn get_command(&self) -> Command;
}
