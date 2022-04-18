use actix_web::dev::Server;
use actix_web::{HttpServer, App, web};
use crate::routes::{subscriptions, health_check};
use std::net::TcpListener;


pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscriptions))
    })
    .listen(listener)?
    .run();
    Ok(server)
}