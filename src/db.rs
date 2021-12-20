pub mod models;
pub mod schema;

use rocket_sync_db_pools::{database, diesel::PgConnection};

#[database("ocdb")]
pub struct OCDBConn(PgConnection);
