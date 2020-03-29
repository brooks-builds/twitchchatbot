use super::schema::chatters;
use diesel::prelude::*;
use diesel::PgConnection;

#[derive(Queryable)]
#[allow(dead_code)]
pub struct Chatter {
    id: i32,
    username: String,
    chat_date: String,
}

#[derive(Insertable)]
#[table_name = "chatters"]
pub struct NewChatter {
    username: String,
}

impl NewChatter {
    pub fn insert(username: String, connection: &PgConnection) {
        diesel::insert_into(chatters::table)
            .values(&chatters::username.eq(username))
            .execute(connection)
            .expect("Error saving chatter");
    }
}
