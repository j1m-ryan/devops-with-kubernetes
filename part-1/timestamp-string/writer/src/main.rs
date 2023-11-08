use std::{
    env,
    fs::OpenOptions,
    io::Write,
    path::Path,
    thread,
    time::{Duration, Instant},
};

use chrono::Utc;

fn main() {
    let seconds = 5;
    let is_dev_mode = env::var("DEV_MODE").unwrap_or("false".to_string()) == "true";

    let path = Path::new(if is_dev_mode {
        "./time.txt"
    } else {
        "/usr/src/app/files/time.txt"
    });

    loop {
        let start = Instant::now();

        let mut file = match OpenOptions::new()
            .append(false)
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
        {
            Err(why) => panic!("couldn't create {}: {}", path.display(), why),
            Ok(file) => file,
        };

        match file.write_all(
            Utc::now()
                .to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
                .as_bytes(),
        ) {
            Err(why) => panic!("couldn't write to {}: {}", path.display(), why),
            Ok(_) => println!("successfully wrote to {}", path.display()),
        }

        if let Some(remaining) = Duration::new(seconds, 0).checked_sub(start.elapsed()) {
            thread::sleep(remaining);
        }
    }
}
