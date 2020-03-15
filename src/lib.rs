#[macro_use]
extern crate diesel;
mod chatbot;
mod database;

use chatbot::chatbot;

pub async fn run(
    twitch_nickname: String,
    twitch_key: String,
    twitch_channel: String,
    database_url: String,
) {
    let database_connection = database::connect(database_url);
    chatbot(
        twitch_nickname,
        twitch_key,
        twitch_channel,
        database_connection,
    )
    .await;
}
