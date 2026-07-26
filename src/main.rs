//! src/main.rs
use std::net::TcpListener;
use secrecy::ExposeSecret;
use sqlx::PgPool;

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
    let connection_pool = PgPool::connect_lazy(
        &configuration.database.connection_string().expose_secret()
    )
    //.await
    .expect("Failed to connect to Postgres.");
    // we have removed the hardcoded 8000, it's now coming from our settings
    let address = format!("{}:{}",
        configuration.application.host,
        configuration.application.port,
    );
    let listener = TcpListener::bind(&address)?;
    println!("http://{}", &address);
    run(listener, connection_pool)?.await
}
