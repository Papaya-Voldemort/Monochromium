mod clipboard;
mod get_path;
mod locked_check;
mod parse_note;
mod password;
mod pretty_notes;
mod slugify;
mod string_check;
mod title;
mod trim;

pub const DATETIME_FORMAT: &str = "%b %-d, %Y at %-I:%M %p";
pub const SHORT_DATETIME_FORMAT: &str = "%b %-d, %-I:%M %p";
pub const DATE_FORMAT: &str = "%b %-d, %Y";
pub const TIME_FORMAT: &str = "%-I:%M %p";

pub use clipboard::{copy, get_paste};
pub use get_path::{PathType, get_path};
pub use locked_check::{is_locked, locked_notice};
pub use parse_note::parse_note;
pub use password::{prompt_lock_password, prompt_unlock_password};
pub use pretty_notes::{pretty_note, pretty_notes};
pub use slugify::slugify;
pub use string_check::string_check;
pub use title::make_title;
pub use trim::trim_text;
