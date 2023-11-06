use axum::{extract::State, routing::get, Router};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct AppState {
    count: Arc<Mutex<usize>>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        count: Arc::new(Mutex::new(0)),
    };

    let app = Router::new().route("/pingpong", get(handler)).with_state(state);

    println!("Listening on port: {}", 3000);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(State(state): State<AppState>) -> String {
    let mut count = state.count.lock().unwrap();
    let formatted = format!("Pong {}", count);
    *count += 1;
    formatted
}
