use std::net::TcpListener;
use reqwest;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind 8000 port");
    let port = listener.local_addr().unwrap().port();
    let server = newsletter::startup::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[actix_rt::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute address");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn subscriptions_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let body = "name=kakshay&email=kakshay%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute address");
    assert_eq!(200, response.status().as_u16());
}

#[actix_rt::test]
async fn subscriptions_return_400_when_data_is_missing() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=kakshay", "missing email"),
        ("email=kakshay%40gmail.com", "missing name"),
        ("", "missing both email and name")
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &address))
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