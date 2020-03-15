pub mod chatter;
pub mod schema;

use diesel::{Connection, PgConnection};

pub fn connect(database_url: String) -> PgConnection {
    PgConnection::establish(&database_url).unwrap()
}
