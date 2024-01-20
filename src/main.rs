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