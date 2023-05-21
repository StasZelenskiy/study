use tokio::sync::mpsc;
use crate::utils::print_header;
use std::sync::Arc;

pub async fn run() {
    print_header("MPSC Channel Example");

    let students_count = 15;
    let (tx, rx) = mpsc::channel(students_count);
    let tx_arc = Arc::new(tx);

    tokio::spawn(check_by_teacher(rx));

    for student in 1..=students_count {
        tokio::spawn(
            process_student_homework(student as i32, tx_arc.clone())
        ).await.unwrap();
    }
}

async fn process_student_homework(id: i32, tx: Arc<mpsc::Sender<String>>) {
    println!("Student {id} is getting their homework");
    tx.send(format!("Student {id}'s homework.")).await.unwrap();
}

async fn check_by_teacher(mut rx: mpsc::Receiver<String>) -> Vec<String> {
    let mut homework = vec![];

    while let Some(student_hw) = rx.recv().await {
        println!("Received homework: {}", &student_hw);
        homework.push(student_hw);
    }

    homework
}
