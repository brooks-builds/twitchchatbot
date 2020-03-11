mod chatbot;

pub struct State {}

impl State {
    pub async fn new(
        twitch_nickname: String,
        twitch_key: String,
        twitch_channel: String,
        database_url: String,
    ) -> State {
        chatbot::main(twitch_nickname, twitch_key, twitch_channel).await;
        State {}
    }
}
