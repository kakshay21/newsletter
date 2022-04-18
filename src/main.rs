use std::net::TcpListener;
use newsletter::startup::run;
use newsletter::configuration::get_configuration;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configurations.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind 8000 port");
    run(listener)?.await
}
