extern crate twitchchatbot;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let nick = env::var("TWITCH_NICK").unwrap();
    let pass = env::var("TWITCH_PASS").unwrap();
    let channel = env::var("TWITCH_CHANNEL").unwrap();

    twitchchatbot::run(nick, pass, channel).await;
}
