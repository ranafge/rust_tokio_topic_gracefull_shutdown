use std::thread::sleep;
use std::time::Duration;

use tokio::signal;
#[tokio::main]
async fn main() {
    // spanw application as seperate task

    match signal::ctrl_c().await {
        Ok(()) => {},
        Err(e) => {
            eprintln!("Unable to listen for shutdown signal: {}", e);
        }
    }
}
// if there is a mutiple shutdown condition use mpsc channel to send the shutdown signal to one place.
// can use select on ctrl_c

// Example

use tokio::sync::mpsc;

// #[tokio::main]
// async fn main() {
//     let (shudown_send, mut shutdown_recv) = mpsc::unbounded_channel();
//     // other task 

//     tokio::select! {
//         _= signal::ctrl_c() => {},
//         _= shutdown_recv.recv() => {}
//     }
// }

use tokio_util::sync::CancellationToken;

#[tokio::main] 

async fn main() {
    let tracker = TaskTracker::new();

    for i in 0..10 {
        tracker.spawn(some_operation(i));
    }
    tracker.close();
    tracker.wait().await;
    println!("tHIS IS PRINTED AFTER ALL OF THE TASKS");
}

async fn some_operation(i: u64) {
    sleep(Duration::from_millis(10));
    println!("Task {} shutting down.", i);
}