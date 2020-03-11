use tokio::stream::StreamExt as _;
use twitchchat::client::Dispatcher;

pub async fn main(nick: String, pass: String, channel: String) {
    let dispatcher = Dispatcher::new();

    // subscribe to a Privmsg event stream
    // whenever the client reads a PRIVMSG, it'll produce an item in this stream
    // you can subscribe multiple times to the same event
    let mut privmsg = dispatcher.subscribe::<twitchchat::events::Privmsg>();

    // spawn a task to consume the stream
    tokio::task::spawn(async move {
        while let Some(msg) = privmsg.next().await {
            eprintln!("[{}] {}: {}", msg.channel, msg.name, msg.data);
        }
    });

    // for join (when a user joins a channel)
    let mut join = dispatcher.subscribe::<twitchchat::events::Join>();
    // for part (when a user leaves a channel)
    let mut part = dispatcher.subscribe::<twitchchat::events::Part>();

    // there is also an `All` event which is an enum of all possible events
    // and a `Raw` event which is the raw IRC message

    // make a new runner
    // control allows you to stop the runner, and gives you access to an async. encoder (writer)
    let (runner, mut control) = twitchchat::Runner::new(dispatcher.clone());

    // connect via TCP with TLS with this nick and password
    let stream = twitchchat::connect_easy_tls(&nick, &pass).await.unwrap();

    // spawn the run off in another task so we don't block the current one.
    // you could just await on the future at the end of whatever block, but this is easier for this demonstration
    let handle = tokio::task::spawn(runner.run(stream));

    // another privmsg so we can act like a bot
    let mut privmsg = dispatcher.subscribe::<twitchchat::events::Privmsg>();

    // we can block on the dispatcher for a specific event
    // if we call one_time again for this event, it'll return the previous one
    eprintln!("waiting for irc ready");
    let ready = dispatcher
        .one_time::<twitchchat::events::IrcReady>()
        .await
        .unwrap();
    eprintln!("our nickname: {}", ready.nickname);

    // we can clone the writer and send it places
    let mut writer = control.writer().clone();

    // because we waited for IrcReady, we can confidently join channels
    writer.join(channel).await.unwrap();

    // a fancy main loop without using tasks
    loop {
        tokio::select! {
            Some(join_msg) = join.next() => {
                eprintln!("{} joined {}", join_msg.name, join_msg.channel);
            }

            Some(part_msg) = part.next() => {
                eprintln!("{} left {}", part_msg.name, part_msg.channel);
            }

            Some(msg) = privmsg.next() => {
                match msg.data.split(" ").next() {
                    Some("!hello") => {
                        let response = format!("hello {}!", msg.name);
                        if let Err(_err) = control.writer().privmsg(&msg.channel, &response).await {
                            // we cannot write, so we should bail
                            break;
                        }
                    }
                    Some("!quit") => {
                        // causes the runner to shutdown
                        control.stop();
                    }
                    _ => {}
                }
            }

            // when the 3 streams in this select are done this'll get hit
            else => { break }
        }
    }

    // you can clear subscriptions with
    // dispatcher.clear_subscriptions::<event::Join>()
    // or all subscriptions
    // dispatcher.clear_subscriptions_all()

    // you can get the number of active subscriptions with
    // dispatcher.count_subscribers::<event::Join>()
    // or all subscriptions
    // dispatcher.count_subscribers_all()

    // await for the client to be done
    // unwrap the JoinHandle
    match handle.await.unwrap() {
        Ok(twitchchat::Status::Eof) => {
            eprintln!("done!");
        }
        Ok(twitchchat::Status::Canceled) => {
            eprintln!("client was stopped by user");
        }
        Err(err) => {
            eprintln!("error: {}", err);
        }
    }

    // note you should wait for all of your tasks to join before exiting
    // but we detached them to make this shorter

    // another way would be to clear all subscriptions
    // clearing the subscriptions would close each event stream
    dispatcher.clear_subscriptions_all();
}
