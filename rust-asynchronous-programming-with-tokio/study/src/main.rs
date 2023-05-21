use example::*;

mod utils;
mod example;

#[tokio::main]
async fn main() {
    mutex::run().await;
    semaphore::run().await;
    notify::run().await;
    barrier::run().await;
    rw_lock::run().await;
    oneshot::run().await;
    mpsc::run().await;
    watch::run().await;
    broadcast::run().await;
}
