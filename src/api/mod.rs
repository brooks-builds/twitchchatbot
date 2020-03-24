use super::database::command;
use warp::Filter;

pub async fn run() {
    let json_reply = warp::reply::with::header("Content-Type", "application/json");
    let get_all = warp::path!("api" / "v1" / "commands")
        .map(|| {
            let commands = command::get_all();
            // let commands = serde_json::to_string(&commands).expect("error serializing commands");

            warp::reply::json(&commands)
        })
        .with(json_reply.clone());

    let get_one = warp::path!("api" / "v1" / "commands" / i32)
        .map(|id| warp::reply::json(&command::get_one(id)))
        .with(json_reply);
    let create = warp::post()
        .and(warp::path!("api" / "v1" / "commands"))
        .and(warp::body::content_length_limit(1024 * 64))
        .and(warp::body::json())
        .map(|new_command: command::NewCommand| {
            if validate_new_command(&new_command.command) {

                let created_command = command::insert(new_command);
                warp::reply::with_status(
                    warp::reply::json(&created_command),
                    warp::http::StatusCode::CREATED,
                )
            } else {
                warp::reply::with_status(warp::reply::json(&serde_json::json!({"error": "New commands must begin with ! and not have any spaces"})), warp::http::StatusCode::BAD_REQUEST)
            }
        });

    let update = warp::put()
        .and(warp::path!("api" / "v1" / "commands" / i32))
        .and(warp::body::content_length_limit(1024 * 64))
        .and(warp::body::json())
        .map(|id: i32, updated_command: command::NewCommand| {
            if validate_new_command(&updated_command.command) {
                let command = command::update(id, updated_command.command, updated_command.response);
                warp::reply::with_status(warp::reply::json(&command), warp::http::StatusCode::OK)
            } else {
                warp::reply::with_status(warp::reply::json(&serde_json::json!({"error": "Commands must begin with ! and not have any spaces"})), warp::http::StatusCode::BAD_REQUEST)
            }
        });

    let routes = warp::get().and(get_all.or(get_one)).or(create).or(update);
    warp::serve(routes).run(([127, 0, 0, 1], 5000)).await;
}

fn validate_new_command(command: &str) -> bool {
    command.starts_with('!') && !command.contains(' ')
}
