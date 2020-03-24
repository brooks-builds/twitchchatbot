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

pub fn get_response(chat_command: &str) -> Option<String> {
    use super::schema::commands::dsl::*;
    let connection = connect();

    let mut result = commands
        .select(response)
        .filter(command.eq(chat_command.to_string()))
        .load::<String>(&connection)
        .expect("error getting response");

    result.pop()
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

pub fn update(id_to_update: i32, new_command: String, new_response: String) -> Command {
    use super::schema::commands::dsl::*;

    let connection = connect();

    diesel::update(commands.filter(id.eq(id_to_update)))
        .set((command.eq(new_command), response.eq(new_response)))
        .get_result(&connection)
        .unwrap()
}

pub fn destroy(id_to_destroy: i32) -> Result<(), String> {
    use super::schema::commands::dsl::*;

    let connection = connect();

    match diesel::delete(commands.filter(id.eq(id_to_destroy))).execute(&connection) {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Error deleting command")),
    }
}
