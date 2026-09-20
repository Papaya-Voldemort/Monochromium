mod clipboard;
mod pretty_notes;
mod string_check;
mod title;
mod parse_note;

pub use clipboard::{copy, get_paste};
pub use pretty_notes::pretty_notes;
pub use string_check::string_check;
pub use title::make_title;
pub use parse_note::parse_note;
