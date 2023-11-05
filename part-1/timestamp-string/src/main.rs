use std::{
    thread,
    time::{Duration, Instant},
};

use chrono::Utc;
use uuid::Uuid;
fn main() {
    let my_uuid = Uuid::new_v4();
    let seconds = 5;

    loop {
        let start = Instant::now();
        let now = Utc::now();

        println!(
            "{} {}",
            now.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            my_uuid.as_simple()
        );

        if let Some(remaining) = Duration::new(seconds, 0).checked_sub(start.elapsed()) {
            thread::sleep(remaining);
        }
    }
}
