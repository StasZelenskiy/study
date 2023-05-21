use tokio::time::{Duration, sleep};
use std::sync::Arc;
use tokio::sync::Semaphore;
use crate::utils::print_header;

pub async fn run() {
    print_header("Semaphore Example");

    let number_of_tellers = 4;
    let semaphore = Semaphore::new(number_of_tellers);
    let semaphore_arc = Arc::new(semaphore);

    let mut people_handles = vec![];
    for num in 0..10 {
        people_handles.push(tokio::spawn(
            person(semaphore_arc.clone(), format!("Person_{num}"))
        ));
    }

    for handle in people_handles {
        handle.await.unwrap();
    }
}

async fn person(e_queue: Arc<Semaphore>, name: String) {
    println!("{} is waiting in line", name);
    teller(e_queue, name).await;

}

async fn teller(e_queue: Arc<Semaphore>, customer: String) {
    let permit = e_queue.acquire().await.unwrap();
    sleep(Duration::from_secs(2)).await;
    println!("{} is being served by the teller", customer);
    sleep(Duration::from_secs(3)).await;
    println!("{} is now leaving the teller", customer);
    drop(permit);
}
