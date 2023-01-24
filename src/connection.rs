use rocket_sync_db_pools::{database, diesel};

#[database("connection")]
pub struct Connection(diesel::MysqlConnection);
