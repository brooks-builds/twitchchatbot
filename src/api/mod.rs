use super::database::command;
use warp::Filter;

pub async fn run(api_key: String) {
    let json_reply = warp::reply::with::header("Content-Type", "application/json");
    let cors = warp::reply::with::header("Access-Control-Allow-Origin", "http://localhost:8080");
    let get_all = warp::path!("api" / "v1" / "commands")
        .map(|| {
            let commands = command::get_all();
            // let commands = serde_json::to_string(&commands).expect("error serializing commands");

            warp::reply::json(&commands)
        })
        .with(json_reply.clone())
        .with(cors.clone());

    let get_one = warp::path!("api" / "v1" / "commands" / i32)
        .map(|id| warp::reply::json(&command::get_one(id)))
        .with(json_reply)
        .with(cors.clone());
    let create_api_key = api_key.clone();
    let create = warp::post()
        .and(warp::path!("api" / "v1" / "commands"))
        .and(warp::header::<String>("api_key"))
        .and(warp::body::content_length_limit(1024 * 64))
        .and(warp::body::json())
        .map(move |header_api_key: String, new_command: command::NewCommand| {
            if create_api_key == header_api_key {
                if validate_new_command(&new_command.command) {                
                    let created_command = command::insert(new_command);
                    warp::reply::with_status(
                        warp::reply::json(&created_command),
                        warp::http::StatusCode::CREATED,
                    )
                } else {
                    warp::reply::with_status(warp::reply::json(&serde_json::json!({"error": "New commands must begin with ! and not have any spaces"})), warp::http::StatusCode::BAD_REQUEST)
                }
            } else {
                warp::reply::with_status(warp::reply::json(&serde_json::json!({"error": "API key missing or incorrect"})), warp::http::StatusCode::UNAUTHORIZED)
            }
            })
            .with(cors.clone());
    let update_api_key = api_key.clone();
    let update = warp::put()
        .and(warp::path!("api" / "v1" / "commands" / i32))
        .and(warp::body::content_length_limit(1024 * 64))
        .and(warp::body::json())
        .and(warp::header::<String>("api_key"))
        .map(move |id: i32, updated_command: command::NewCommand, header_api_key: String| {
            if header_api_key == update_api_key {

                if validate_new_command(&updated_command.command) {
                    let command = command::update(id, updated_command.command, updated_command.response);
                    warp::reply::with_status(warp::reply::json(&command), warp::http::StatusCode::OK)
                } else {
                    warp::reply::with_status(warp::reply::json(&serde_json::json!({"error": "Commands must begin with ! and not have any spaces"})), warp::http::StatusCode::BAD_REQUEST)
                }
            } else {
                warp::reply::with_status(warp::reply::json(&serde_json::json!({"error": "API key missing or incorrect"})), warp::http::StatusCode::UNAUTHORIZED)
            }
        }).with(cors.clone());
    let destroy_api_key = api_key.clone();
    let destroy = warp::delete()
        .and(warp::path!("api" / "v1" / "commands" / i32))
        .and(warp::header::<String>("api_key"))
        .map(move |id, header_api_key: String| {
            if destroy_api_key == header_api_key {
                match command::destroy(id) {
                    Ok(_) => warp::reply::with_status(
                        warp::reply::json(&serde_json::json!({"status": "success"})),
                        warp::http::StatusCode::OK,
                    ),
                    Err(message) => warp::reply::with_status(
                        warp::reply::json(&serde_json::json!({ "error": message })),
                        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                    ),
                }
            } else {
                warp::reply::with_status(warp::reply::json(&serde_json::json!({"error": "API key missing or incorrect"})), warp::http::StatusCode::UNAUTHORIZED)
            }
        }).with(cors.clone());

    let routes = warp::get()
        // .and(authorize)
        .and(get_all.or(get_one))
        .or(create)
        .or(update)
        .or(destroy);
    warp::serve(routes).run(([0, 0, 0, 0], 5000)).await;
}

fn validate_new_command(command: &str) -> bool {
    command.starts_with('!') && !command.contains(' ')
}
