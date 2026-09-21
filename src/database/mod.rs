mod add;
mod db;
mod delete;
mod edit;
mod read;
mod search;
#[cfg(test)]
mod tests;

pub struct Database {
    pub(crate) conn: rusqlite::Connection,
}

pub use edit::UpdateType;
