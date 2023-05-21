use tokio::sync::oneshot;
use tokio::time::{sleep, Duration};
use crate::utils::print_header;

pub async fn run() {
    print_header("Oneshot Channel Example");

    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(spy_agency(tx1, "MI-6")).await.unwrap();
    tokio::spawn(spy_agency(tx2, "CIA")).await.unwrap();
    sleep(Duration::from_secs(1)).await;
    tokio::spawn(double_agent(rx1, rx2)).await.unwrap();
}

async fn double_agent(
    mut rx1: oneshot::Receiver<String>,
    mut rx2: oneshot::Receiver<String>
) {
    let msg = tokio::select! {
        msg1 = &mut rx1 => msg1.unwrap(),
        msg2 = &mut rx2 => msg2.unwrap(),
    };

    println!("Spy has received the following task: {msg}");
}

async fn spy_agency(tx: oneshot::Sender<String>, id: &str) {
    tx.send(format!("Here is your {id} task").to_string()).unwrap();
    println!("{id} sent message");
}
