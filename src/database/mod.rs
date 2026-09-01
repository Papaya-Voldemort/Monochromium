mod db;
mod add;
mod read;
mod delete;

pub use db::make_db;
pub use add::add_row;
pub use read::read_rows;
pub use delete::delete_note;