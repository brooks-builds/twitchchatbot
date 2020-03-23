use super::connect;
use super::schema::commands;
use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, Deserialize, Serialize)]
pub struct Command {
    id: i32,
    command: String,
    response: String,
    used: i32,
}

#[derive(Deserialize, Serialize, Insertable)]
#[table_name = "commands"]
pub struct NewCommand {
    pub command: String,
    pub response: String,
}

pub fn get_response(chat_command: &str, connection: &diesel::PgConnection) -> String {
    use super::schema::commands::dsl::*;

    commands
        .select(response)
        .filter(command.eq(chat_command.to_string()))
        .first::<String>(connection)
        .expect("error getting response")
}

pub fn get_all() -> Vec<Command> {
    use super::schema::commands::dsl::*;

    let database_url = std::env::var("DATABASE_URL").unwrap();
    let connection = connect();

    commands
        .load::<Command>(&connection)
        .expect("error getting all commands")
}

pub fn get_one(command_id: i32) -> Command {
    use super::schema::commands::dsl::*;

    let database_url = std::env::var("DATABASE_URL").unwrap();
    let connection = connect();

    commands
        .filter(id.eq(command_id))
        .first::<Command>(&connection)
        .expect("error loading one command")
}

pub fn insert(new_command: NewCommand) -> Command {
    let connection = connect();

    diesel::insert_into(commands::table)
        .values(&new_command)
        .get_result(&connection)
        .unwrap()
}
