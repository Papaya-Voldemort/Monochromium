mod clipboard;
mod get_path;
mod parse_note;
mod pretty_notes;
mod string_check;
mod title;
mod trim;

pub use clipboard::{copy, get_paste};
pub use get_path::{PathType, get_path};
pub use parse_note::parse_note;
pub use pretty_notes::pretty_notes;
pub use string_check::string_check;
pub use title::make_title;
pub use trim::trim_text;
