mod delete;
mod edit;
mod read;
mod write;
mod check;

pub use delete::{delete_file, shred_and_delete};
pub use edit::edit_file;
pub use read::{read_file, read_file_if_exists};
pub use write::write_all;
pub use check::check_file_type;