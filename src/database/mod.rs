mod add;
mod db;
mod delete;
mod edit;
mod read;
mod search;
#[cfg(test)]
mod tests;
mod migration;



pub struct Database {
    pub(crate) conn: rusqlite::Connection,
}
