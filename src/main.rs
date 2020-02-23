use std::env;
use tokio::stream::StreamExt as _;
use twitchchat::{
    events::{Join, Privmsg},
    Client, Secure,
};

#[tokio::main]
async fn main() {
    let nick = env::var("TWITCH_NICK").unwrap();
    let pass = env::var("TWITCH_PASS").unwrap();
    let channel = env::var("TWITCH_CHANNEL").unwrap();

    let (read, write) = twitchchat::connect_easy(&nick, &pass, Secure::UseTls)
        .await
        .unwrap();

    let client = Client::new();

    let done = client.run(read, write);

    // let mut private_message = client.dispatcher().await.subscribe::<Privmsg>();

    // tokio::task::spawn(async move {
    //     while let Some(message) = private_message.next().await {
    //         dbg!(message);
    //     }
    // });

    // let mut join = client
    //     .dispatcher()
    //     .await
    //     .subscribe::<twitchchat::events::Join>();

    // tokio::task::spawn(async move {
    //     while let Some(msg) = join.next().await {
    //         // we've joined a channel
    //         println!("someone joined us in chat: {:?}", msg);
    //     }
    // });

    let mut bot = client
        .dispatcher()
        .await
        .subscribe::<twitchchat::events::Privmsg>();

    // we can move the client to another task by cloning it
    let bot_client = client.clone();
    tokio::task::spawn(async move {
        let mut writer = bot_client.writer();
        while let Some(msg) = bot.next().await {
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

    if let Err(err) = client.writer().join(&channel).await {
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
