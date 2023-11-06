use std::{
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use chrono::Utc;
use uuid::Uuid;

use axum::{routing::get, Router};
use tokio::task;

#[tokio::main]
async fn main() {
    let my_uuid = Arc::new(Uuid::new_v4());
    let seconds = 5;

    let uuid_for_background = my_uuid.clone();
    task::spawn(async move {
        loop {
            let start = Instant::now();

            println!("{}", get_current_status(&uuid_for_background));

            if let Some(remaining) = Duration::new(seconds, 0).checked_sub(start.elapsed()) {
                thread::sleep(remaining);
            }
        }
    });

    let app = Router::new().route(
        "/",
        get(move || {
            let uuid = my_uuid.clone();
            async move { get_current_status(&uuid) }
        }),
    );

    println!("Listening on port {}", 3000);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn get_current_status(uuid: &Uuid) -> String {
    let now = Utc::now();
    format!(
        "{} {}",
        now.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        uuid.as_simple()
    )
}
