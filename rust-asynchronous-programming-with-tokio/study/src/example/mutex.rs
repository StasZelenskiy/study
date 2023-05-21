use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use std::sync::Arc;
use crate::utils::print_header;

pub async fn run() {
    print_header("Mutex Example");

    let tv_channel = 10;
    let remote = Mutex::new(tv_channel);
    let remote_arc = Arc::new(remote);

    let mut task_handles = vec![];

    for (name, new_channel) in [("Stas", 1), ("Sveta", 8), ("Albert", 5)] {
        task_handles.push(
            tokio::spawn(
                person(
                    remote_arc.clone(),
                    name.to_string(),
                    new_channel)))
    }

    for handle in task_handles {
        handle.await.unwrap();
    }
}

async fn person(remote: Arc<Mutex<i32>>, name: String, new_channel: i32) {
    let mut real_remote = remote.lock().await;

    *real_remote = new_channel;
    println!("{} changed the channel", name);
    println!("Watching channel {}", new_channel);

    sleep(Duration::from_secs(2)).await;
}
