extern crate twitchchatbot;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let nick = env::var("TWITCH_NICK").unwrap();
    let pass = env::var("TWITCH_PASS").unwrap();
    let channel = env::var("TWITCH_CHANNEL").unwrap();
    let database_url = env::var("DATABASE_URL").unwrap();
    let api_key = env::var("API_KEY").unwrap();

    twitchchatbot::run(nick, pass, channel, database_url, api_key).await;
}
