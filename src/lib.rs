#[macro_use]
extern crate diesel;
mod api;
mod chatbot;
mod database;

use chatbot::chatbot;

pub async fn run(
    twitch_nickname: String,
    twitch_key: String,
    twitch_channel: String,
    database_url: String,
    api_key: String,
) {
    let api_join_handle = tokio::task::spawn(async {
        api::run(api_key).await;
    });
    let chatbot_join_handle = tokio::task::spawn(async {
        chatbot(twitch_nickname, twitch_key, twitch_channel).await;
    });

    api_join_handle.await.expect("api error");
    chatbot_join_handle.await.expect("chatbot error");
}
