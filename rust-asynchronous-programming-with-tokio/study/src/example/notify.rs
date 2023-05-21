use tokio::sync::{Notify};
use tokio::time::{sleep, Duration};
use std::sync::Arc;
use crate::utils::print_header;

pub async fn run() {
    print_header("Notify Example");

    let package_delivered = Notify::new();
    let package_delivered_arc = Arc::new(package_delivered);

    let order_package_handle = tokio::spawn(
        order_package(package_delivered_arc.clone())
    );

    let grap_package_handle = tokio::spawn(
        grab_package(package_delivered_arc.clone())
    );

    order_package_handle.await.unwrap();
    grap_package_handle.await.unwrap();
}

async fn order_package(package_delivered: Arc<Notify>) {
    sleep(Duration::from_secs(2)).await;
    println!("Find package");

    sleep(Duration::from_secs(2)).await;
    println!("Ship package");

    sleep(Duration::from_secs(2)).await;
    println!("Package has been delivered");    
    package_delivered.notify_one();
}

async fn grab_package(package_delivered: Arc<Notify>) {
    package_delivered.notified().await;
    println!("Look outside house for package");
    sleep(Duration::from_secs(2)).await;
    println!("Grab package");
}
