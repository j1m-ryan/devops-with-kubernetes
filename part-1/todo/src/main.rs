use std::{fs::File, io::Read};

use axum::{
    body::Full,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(serve_index));
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    println!("Server started in port {}", port);

    axum::Server::bind(&format!("0.0.0.0:{}", port).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn serve_index() -> impl IntoResponse {
    match File::open("./public/index.html") {
        Ok(mut file) => {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                Response::new(Full::from(contents))
            } else {
                eprintln!("Error reading index.html file");
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Full::from("Internal Server Error"))
                    .unwrap()
            }
        }
        Err(_) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Full::from("404 Not Found"))
            .unwrap(),
    }
}
