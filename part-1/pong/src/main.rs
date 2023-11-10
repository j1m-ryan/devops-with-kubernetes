use axum::{
    extract::{path, State},
    routing::get,
    Router,
};
use std::{
    env,
    fs::OpenOptions,
    io::Write,
    path::PathBuf,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
struct AppState {
    path: PathBuf,
}

#[tokio::main]
async fn main() {
    let is_dev_mode = env::var("DEV_MODE").unwrap_or("false".to_string()) == "true";
    let pathbuf: PathBuf = PathBuf::from(if is_dev_mode {
        "./state.txt"
    } else {
        "/tmp/kube/state.txt"
    });
    let state = AppState { path: pathbuf };

    let app = Router::new()
        .route("/pingpong", get(handler))
        .with_state(state);

    println!("Listening on port: {}", 3000);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(State(state): State<AppState>) -> String {
    let path_clone = state.path.clone();
    let mut file = match OpenOptions::new()
        .append(false)
        .create(true)
        .write(true)
        .open(state.path)
    {
        Ok(file) => file,
        Err(e) => panic!("Error opening file: {}", e),
    };
    let content = match std::fs::read_to_string(path_clone) {
        Ok(content) => content,
        Err(e) => panic!("Error reading file: {}", e),
    };
    let count = match content.parse::<i32>() {
        Ok(count) => count,
        Err(_e) => 0,
    };
    let formatted = format!("Pong {}", count);

    let new_count = count + 1;
    match file.write_all(new_count.to_string().as_bytes()) {
        Ok(_) => (),
        Err(e) => panic!("Error writing to file: {}", e),
    };

    formatted
}
