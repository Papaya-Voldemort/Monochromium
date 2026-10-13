mod delete;
mod edit;
mod read;
mod write;

pub use delete::{delete_file, shred_and_delete};
pub use edit::edit_file;
pub use read::{read_file, read_file_if_exists};
pub use write::write_all;
