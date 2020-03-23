pub mod chatter;
pub mod command;
pub mod schema;

use diesel::{Connection, PgConnection};
use std::env;

pub fn connect() -> PgConnection {
    let database_url = env::var("DATABASE_URL").unwrap();
    PgConnection::establish(&database_url).unwrap()
}
