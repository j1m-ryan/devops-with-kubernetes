use axum::{routing::get, Router};
#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello" }));
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    println!("Server started in port {}", port);

    axum::Server::bind(&format!("0.0.0.0:{}", port).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
