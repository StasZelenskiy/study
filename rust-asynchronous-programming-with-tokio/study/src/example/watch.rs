use tokio::sync::watch;
use tokio::time::{sleep, Duration};
use crate::utils::print_header;

pub async fn run() {
    print_header("Watch Channel Example");

    let flight_status = String::from("On time");

    let (airline, _) = watch::channel(flight_status);
    let passenger1 = airline.subscribe();
    let passenger2 = airline.subscribe();

    tokio::spawn(listen_for_flight_status(passenger1, 1));
    tokio::spawn(listen_for_flight_status(passenger2, 2));

    airline.send("On time".to_string()).unwrap();
    sleep(Duration::from_secs(1)).await;
    airline.send("Delayed 15 minutes".to_string()).unwrap();
    sleep(Duration::from_secs(1)).await;
    airline.send("Delayed 30 minutes".to_string()).unwrap();
    sleep(Duration::from_secs(1)).await;
}

async fn listen_for_flight_status(mut passenger: watch::Receiver<String>, id: i32) {
    while passenger.changed().await.is_ok() {
        let new_status = passenger.borrow().clone();
        println!("Passenger {id} got flight status updated: {new_status}");
    }
}
