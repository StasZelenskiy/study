#[allow(dead_code)]

use server::Server;
use website_handler::WebsiteHandler;
use std::env;

mod http;
mod server;
mod handler;
mod website_handler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    // let public_path = fs::canonicalize(env::var("PUBLIC_PATH").unwrap_or(default_path))
    //     .unwrap()
    //     .into_os_string()
    //     .into_string()
    //     .unwrap();
    println!("Public path: {}", public_path);

    let server = Server::new(String::from("127.0.0.1:8080"));
    server.run(WebsiteHandler::new(&public_path));
}
