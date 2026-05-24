//! src/lib.rs

use actix_web::{App, HttpResponse, HttpServer, Responder, dev::Server, web};
use std::net::TcpListener;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

// // We need to mark `run` as public.
// // It is no longer a binary entrypoint, therefore we can mark it as async
// // without having to use any proc-macro incantation.
// pub async fn run() -> Result<(), std::io::Error> {
// HttpServer::new( || {
//             App::new()
//             .route("/health_check", web::get().to(health_check))
//     })
//     .bind("127.0.0.1:8000")?
//     .run()
//     .await
// }

// Notice the different signature!
// We return `Server` on the happy path and we dropped the `async` keyword
// We have no .await call, so it is not needed anymore.
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new( || {
        App::new()
        .route("health_check", web::get().to(health_check))
    })
    .listen(listener)?
    .run();
    Ok(server)
}