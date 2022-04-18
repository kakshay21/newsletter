use actix_web::dev::Server;
use actix_web::{HttpServer, App, web};
use crate::routes::{subscriptions, health_check};
use std::net::TcpListener;
use sqlx::PgPool;


pub fn run(
    listener: TcpListener,
    db_pool: PgPool
) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscriptions))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}