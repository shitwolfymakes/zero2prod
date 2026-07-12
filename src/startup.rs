//! src/startup.rs
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use crate::routes::{health_check, subscribe};

// Notice the different signature!
// We return `Server` on the happy path and we dropped the `async` keyword
// We have no .await call, so it is not needed anymore.
pub fn run(
    listener: TcpListener,
    db_pool: PgPool
) -> Result<Server, std::io::Error> {
    // wrap the connection in a smart pointer
    let connection = web::Data::new(db_pool);
    // capture `connection` from the surrounding environment
    let server = HttpServer::new(move || {
        App::new()
        .route("health_check", web::get().to(health_check))
        .route("/subscriptions", web::post().to(subscribe))
        // Register the connection as part of the application state
        .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}