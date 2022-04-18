use std::net::TcpListener;
use reqwest;
use sqlx::{PgPool, PgConnection, Connection, Executor};
use newsletter::configuration::{get_configuration, DataBaseSettings};
use uuid::Uuid;

pub struct TestApp {
    pub db_pool: PgPool,
    pub address: String
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind 8000 port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read configurations");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(&configuration.database).await;
    
    let server = newsletter::startup::run(listener, connection_pool.clone())
        .expect("Failed to bind address");
    let _ = tokio::spawn(server);
    
    TestApp {
        db_pool: connection_pool,
        address
    }
}

pub async fn configure_database(config: &DataBaseSettings) -> PgPool {
    let mut connect = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to DB.");
    connect.execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create a database.");

    let db_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to DB.");
    
    sqlx::migrate!()
        .run(&db_pool)
        .await
        .expect("Failed to migrate the DB.");
    
    db_pool
}

#[actix_rt::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute address");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn subscriptions_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let body = "name=kakshay&email=kakshay%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute address");
    assert_eq!(200, response.status().as_u16());

    let record = sqlx::query!("SELECT email, name from subscriptions")
        .fetch_one(&app.db_pool)    
        .await
        .expect("Failed to fetch saved subscriptions.");
    assert_eq!(record.email, "kakshay@gmail.com");
    assert_eq!(record.name, "kakshay");
}

#[actix_rt::test]
async fn subscriptions_return_400_when_data_is_missing() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=kakshay", "missing email"),
        ("email=kakshay%40gmail.com", "missing name"),
        ("", "missing both email and name")
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute address");
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 BAD Request when the payload was {}", error_message
        );
    }
    
}