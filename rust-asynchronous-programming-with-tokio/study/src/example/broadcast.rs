use tokio::sync::broadcast::{self, error::RecvError};
use tokio::time::{sleep, Duration};
use crate::utils::print_header;

pub async fn run() {
    print_header("Broadcast Channel Example");

    let (tx, _) = broadcast::channel(50);
    let bob = tx.clone();
    let alice = tx.clone();

    tokio::spawn(chat(tx.subscribe()));
    tokio::spawn(participant("Bob", bob.clone(), tx.subscribe()));
    tokio::spawn(participant("Alice", alice.clone(), tx.subscribe()));
    sleep(Duration::from_secs(1)).await;

    bob.send(Message::new("Bob", "Hello!")).unwrap();
    sleep(Duration::from_secs(1)).await;
    alice.send(Message::new("Alice", "Hello!")).unwrap();
    sleep(Duration::from_secs(1)).await;
    bob.send(Message::new("Bob", "Alice, how are you?")).unwrap();
    sleep(Duration::from_secs(1)).await;
    alice.send(Message::new("Alice", "Bob, how is the weather?!")).unwrap();
    sleep(Duration::from_secs(1)).await;
}

#[derive(Debug, Clone)]
struct Message {
    pub sender: String,
    pub message: String,
}

impl Message {
    pub fn new(from: &str, message: &str) -> Self {
        Self {
            sender: from.to_string(),
            message: message.to_string()
        }
    }
}

async fn chat(mut rx: broadcast::Receiver<Message>) {
    loop {
        match rx.recv().await {
            Ok(msg) => println!("{}: {}", msg.sender, msg.message),
            Err(RecvError::Lagged(_missed)) => break,
            Err(RecvError::Closed) => break,
        }
    }
}

async fn participant(name: &str, tx: broadcast::Sender<Message>, mut rx: broadcast::Receiver<Message>) {
    tx.send(Message::new(name, &format!("is entering chat"))).unwrap();
    loop {
        match rx.recv().await {
            Ok(msg) => {
                if !msg.sender.eq_ignore_ascii_case(name)
                    && msg.message.contains(name) {
                    tx.send(Message::new(name, &format!("Reply on: {}", msg.message))).unwrap();
                }
            },
            Err(RecvError::Lagged(missed)) => {
                println!("{name} missed {missed} messages");
            },
            Err(RecvError::Closed) => {
                println!("{name} is leaving chat, because it is closed");
                break
            },
        }
    }
}
