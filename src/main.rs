//! src/main.rs

use zero2prod::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Bubble up the io:Error if we failed to bind the address
    // Otherwise call await on our Server
    let listener = TcpListener::bind("127.0.0.1:8000")
        .expect("Failed to bind to random port");
    // We retrieve the port assigned to us by the OS
    // let port = listener.local_addr().unwrap().port();
    println!("http://127.0.0.1:{}", 8000);
    run(listener)?.await
}
