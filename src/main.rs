//! src/main.rs
use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration");
    // we have removed the hardcoded 8000, it's now coming from our settings
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(&address)?;
    println!("http://{}:{}", &address, configuration.application_port);
    run(listener)?.await
}
