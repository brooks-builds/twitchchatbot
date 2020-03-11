extern crate twitchchatbot;

use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let twitch_nickname =
        env::var("TWITCH_NICKNAME").expect("environment variable TWITCH_NICKNAME missing");
    let twitch_key = env::var("TWITCH_KEY").expect("environment variable TWITCH_KEY missing");
    let twitch_channel =
        env::var("TWITCH_CHANNEL").expect("environment variable TWITCH_CHANNEL missing");
    let database_url = env::var("DATABASE_URL").expect("environment variable DATABASE_URL missing");

    let _state =
        twitchchatbot::State::new(twitch_nickname, twitch_key, twitch_channel, database_url).await;
}
