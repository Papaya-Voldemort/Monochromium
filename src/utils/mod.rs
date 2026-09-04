mod clipboard;
mod parse_type;
mod string_check;
mod title;

pub use clipboard::{copy, get_paste};
pub use parse_type::parse_note_type;
pub use string_check::string_check;
pub use title::make_title;
