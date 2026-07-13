//! src/main.rs
use std::net::TcpListener;
use env_logger::Env;
use sqlx::PgPool;
use tracing::subscriber::{self, set_global_default};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // we removed the `env_logger`

    // we are falling back to printing all spans at info-level or above
    // if the RUST_LOG environment variable has not been set
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new(
        "zero2prod".into(),
        // output formatted spans to stdout
        std::io::stdout,
    );
    // the `with` method is provided by `SubscriberExt`, an extension
    // trait for `Subscriber` exposed by `tracing_subscriber`
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    // `set_global_default`` can be used by applications to specify
    // what subscriber should be used to process spans
    set_global_default(subscriber).expect("Failed to set subscriber");

    // panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration");

    let connection_pool = PgPool::connect(
        &configuration.database.connection_string()
    )
    .await
    .expect("Failed to connect to Postgres.");
    // we have removed the hardcoded 8000, it's now coming from our settings
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(&address)?;
    println!("http://{}", &address);
    run(listener, connection_pool)?.await
}
