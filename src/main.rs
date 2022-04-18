use std::net::TcpListener;
use newsletter::startup::run;
use newsletter::configuration::get_configuration;
use sqlx::PgPool;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configurations.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind 8000 port");
    let connection_string = configuration.database.connection_string();
    let connection_pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to DB");
    run(listener, connection_pool)?.await
}
