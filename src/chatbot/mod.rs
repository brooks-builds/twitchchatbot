use super::database::chatter::NewChatter;
use diesel::PgConnection;
use tokio::stream::StreamExt as _;
use twitchchat::{
    events::{Join, Privmsg},
    Client, Secure,
};

pub async fn chatbot(
    twitch_nickname: String,
    twitch_key: String,
    twitch_channel: String,
    database_connection: PgConnection,
) {
    let (read, write) = twitchchat::connect_easy(&twitch_nickname, &twitch_key, Secure::UseTls)
        .await
        .unwrap();

    let client = Client::new();

    let done = client.run(read, write);

    let mut bot = client
        .dispatcher()
        .await
        .subscribe::<twitchchat::events::Privmsg>();

    let bot_client = client.clone();
    tokio::task::spawn(async move {
        let mut writer = bot_client.writer();
        while let Some(msg) = bot.next().await {
            super::database::chatter::NewChatter::insert(
                msg.name.to_string(),
                &database_connection,
            );
            // get the command from the message
            // check the database to see if we have the command
            // respond with the command response or an error
            match msg.data.split(" ").next() {
                Some("!quit") => {
                    // causes the client to shutdown
                    bot_client.stop().await.unwrap();
                }
                Some("!hello") => {
                    let response = format!("hello {}!", msg.name);
                    // send a message in response
                    if let Err(_err) = writer.privmsg(&msg.channel, &response).await {
                        // we ran into a write error, we should probably leave this task
                        return;
                    }
                }
                _ => {}
            }
        }
    });

    if let Err(err) = client.writer().join(&twitch_channel).await {
        match err {
            twitchchat::client::Error::InvalidChannel(..) => {
                eprintln!("you cannot join a channel with an empty name. demo is ending");
                std::process::exit(1);
            }
            _ => {
                // we'll get an error if we try to write to a disconnected client.
                // if this happens, you should shutdown your tasks
            }
        }
    }

    match done.await {
        Ok(twitchchat::client::Status::Eof) => {
            eprintln!("done!");
        }
        Ok(twitchchat::client::Status::Canceled) => {
            eprintln!("client was stopped by user");
        }
        Err(err) => {
            eprintln!("error: {}", err);
        }
    }
}
