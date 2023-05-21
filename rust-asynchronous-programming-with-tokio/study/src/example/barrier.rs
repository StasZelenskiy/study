use tokio::sync::{Barrier, BarrierWaitResult, Notify};
use tokio::time::{sleep, Duration};
use std::sync::Arc;
use crate::utils::print_header;

pub async fn run() {
    print_header("Barrier Example");

    let total_cans_needed = 12;
    let packing_gate = Arc::new(Barrier::new(total_cans_needed));
    let packing_line_is_empty = Arc::new(Notify::new());

    let mut task_handles = vec![];

    packing_line_is_empty.notify_one();

    for can_counts in 0..60 {
        println!("Can #{}: produced", can_counts + 1);
        if can_counts % 12 == 0 {
            // wait till we can produce another 12 cans
            packing_line_is_empty.notified().await;
            println!(">>> Packing gate is open");
            sleep(Duration::from_secs(1)).await;
        }

        task_handles.push(tokio::spawn(
            move_to_packing_line(can_counts + 1, packing_gate.clone(), packing_line_is_empty.clone())
        ));
    }

    let mut num_of_leaders = 0;

    for handle in task_handles {
        let wait_result = handle.await.unwrap();

        if wait_result.is_leader() {
            num_of_leaders += 1;
        }
    }

    println!(">>> Num of leaders: {num_of_leaders}");
}

async fn move_to_packing_line(
    can_id: i32,
    packing_gate: Arc<Barrier>,
    packing_line_is_empty: Arc<Notify>
) -> BarrierWaitResult {
    println!("Can #{can_id}: waiting at packing gate");

    let wait_result = packing_gate.wait().await;
    println!("Can #{can_id}: passed through the packing gate");

    if wait_result.is_leader() {
        packing_line_is_empty.notify_one();
    }

    wait_result
}
