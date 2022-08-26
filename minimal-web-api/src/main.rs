use std::ptr::hash;
use std::thread::sleep;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};
use tokio::task;
use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path!("hello" / String / String)
        .map(|name, tag| format!("Hello, {} + {}", name, tag));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;

    // let sha_printer_task = task::spawn(async {
    //     loop {
    //         let mut hasher = Sha256::new();
    //
    //         let random = rand::random::<i32>();
    //         hasher.update(format!("random-number-{}", random));
    //
    //         let result = hasher.finalize();
    //
    //         let timestamp = std::time::SystemTime::now();
    //         let utc: DateTime<Utc>= timestamp.into();
    //
    //         println!("{} - {:x}", utc.to_rfc3339(), result);
    //         sleep(Duration::from_secs(5));
    //     }
    // });
    //
    // sha_printer_task.await;
}