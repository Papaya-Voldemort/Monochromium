mod add;
mod db;
mod delete;
mod edit;
mod read;
mod search;

pub struct Database {
    pub(crate) conn: rusqlite::Connection,
}

pub use edit::UpdateType;
