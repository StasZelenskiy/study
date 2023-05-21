use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};
use std::sync::Arc;
use crate::utils::print_header;

pub async fn run() {
    print_header("RW Lock Example");

    let doc = Arc::new(RwLock::new("".to_string()));

    let mut handles = vec![];

    for new_string in "I can read and write a b c d e f g h i j".split(" ") {
        handles.push(tokio::spawn(read_from_doc(1, doc.clone())));
        handles.push(tokio::spawn(write_to_doc(doc.clone(), new_string)));
        sleep(Duration::from_millis(5)).await;
        handles.push(tokio::spawn(read_from_doc(2, doc.clone())));
        handles.push(tokio::spawn(read_from_doc(3, doc.clone())));
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

async fn read_from_doc(id: i32, doc: Arc<RwLock<String>>) {
    let reader = doc.read().await;
    println!("Reader #{} is reading the next idea: {}", id, reader);
}

async fn write_to_doc(doc: Arc<RwLock<String>>, new_string: &str) {
    let mut writer = doc.write().await;
    println!("I will write new ideas!");
    writer.push_str(new_string);
    writer.push_str(" ");
}
