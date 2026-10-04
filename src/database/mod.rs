mod add;
mod db;
mod delete;
mod edit;
mod migration;
mod read;
mod search;
#[cfg(test)]
mod tests;

pub struct Database {
    pub(crate) conn: rusqlite::Connection,
}
