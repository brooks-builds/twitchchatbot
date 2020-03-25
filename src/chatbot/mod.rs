use super::database::chatter::NewChatter;
use diesel::PgConnection;
use tokio::stream::StreamExt as _;
use twitchchat::{
    events::{Join, Privmsg},
    Client, Secure,
};

pub async fn chatbot(twitch_nickname: String, twitch_key: String, twitch_channel: String) {
    let database_connection = super::database::connect();
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
            if let Some(command) = extract_command(&msg.data) {
                let response = match super::database::command::get_response_by_command(command) {
                    Ok(response) => response,
                    Err(message) => message.to_string(),
                };
                if let Err(_err) = writer.privmsg(&msg.channel, &response).await {
                    // we ran into a write error, we should probably leave this task
                    return;
                }
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

fn extract_command(message: &str) -> Option<&str> {
    if message.starts_with('!') {
        return message.split(' ').next();
    }

    None
}
