use super::schema::commands;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Command {
    id: i32,
    command: String,
    response: String,
    used: i32,
}

pub fn get_response(chat_command: &str, connection: &diesel::PgConnection) -> String {
    use super::schema::commands::dsl::*;

    commands
        .select(response)
        .filter(command.eq(chat_command.to_string()))
        .first::<String>(connection)
        .expect("error getting response")
}
