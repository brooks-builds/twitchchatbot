mod chatbot;

use chatbot::chatbot;

pub async fn run(twitch_nickname: String, twitch_key: String, twitch_channel: String) {
    chatbot(twitch_nickname, twitch_key, twitch_channel).await;
}
