//! src/main.rs
use std::net::TcpListener;

use sqlx::postgres::PgPoolOptions;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber(
        "zero2prod".into(), "info".into(), std::io::stdout
    );
    init_subscriber(subscriber);

    // panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration");

    // no longer async, given that we don't actually try to connect
    let connection_pool = PgPoolOptions::new()
        .connect_lazy_with(configuration.database.with_db());
    
    // we have removed the hardcoded 8000, it's now coming from our settings
    let address = format!("{}:{}",
        configuration.application.host,
        configuration.application.port,
    );
    let listener = TcpListener::bind(&address)?;
    println!("http://{}", &address);
    run(listener, connection_pool)?.await
}
