use axum::{routing::get, Router};
use std::{env, fs::File, io::Read, path::PathBuf, sync::Arc};

#[tokio::main]
async fn main() {
    let my_uuid = Arc::new(uuid::Uuid::new_v4());
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or(".".to_string());
    let is_dev_mode = env::var("DEV_MODE").unwrap_or_else(|_| "false".to_string()) == "true";

    let path = if is_dev_mode {
        PathBuf::from(format!("{}/../writer/time.txt", manifest_dir))
    } else {
        PathBuf::from("/usr/src/app/files/time.txt")
    };

    let pong_path = if is_dev_mode {
        PathBuf::from(format!("{}/../../pong/state.txt", manifest_dir))
    } else {
        PathBuf::from("/tmp/kube/state.txt")
    };

    let app = Router::new().route(
        "/",
        get(move || {
            let path = path.clone();
            let my_uuid = my_uuid.clone();
            async move {
                let file = File::open(&path);
                let pong_file = File::open(&pong_path);
                match file {
                    Ok(file) => match pong_file {
                        Ok(pong_file) => {
                            let mut reader: std::io::BufReader<File> =
                                std::io::BufReader::new(file);
                            let mut contents = String::new();
                            reader.read_to_string(&mut contents).unwrap();

                            let mut pong_reader = std::io::BufReader::new(pong_file);
                            let mut pong_contents = String::new();
                            pong_reader.read_to_string(&mut pong_contents).unwrap();
                            format!(
                                "{}: {}\nPing / Pongs: {} ",
                                my_uuid, contents, pong_contents
                            )
                        }
                        Err(e) => {
                            println!("Error: {}", e);
                            let mut reader: std::io::BufReader<File> =
                                std::io::BufReader::new(file);
                            let mut contents = String::new();
                            reader.read_to_string(&mut contents).unwrap();

                            format!("{}: {}\nPing / Pongs: {} ", my_uuid, contents, 0)
                        }
                    },
                    Err(e) => {
                        println!("Error: {}", e);
                        format!("{}: {}", my_uuid, e)
                    }
                }
            }
        }),
    );

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
