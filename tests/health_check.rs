use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Fail to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Fail to bind address");
    tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", address))
        .send()
        .await
        .expect("Fail to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0u64), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app_address = spawn_app();
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connect_string = configuration.database.connection_string();
    let mut connection = PgConnection::connect(&connect_string)
        .await
        .expect("Failed to connect to Postgres.");

    let saved = sqlx::query!("SELECT name, email FROM subscriptions")
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription.");

    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com"; // %20 is space, %40 is @ in url

    let response = client
        .post(&format!("{}/subscriptions", app_address))
        .header("Content-Type", "application/x-www-form-urlencoded") // application/x-www-form-urlencoded means body is one giant query string using "&" and "="
        .body(body)
        .send()
        .await
        .expect("Fail to execute request");

    assert_eq!(response.status(), 200);
    assert_eq!(saved.name, "le guin");
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Fail to execute request");

        assert_eq!(
            response.status(),
            400,
            "The API did not fail with 400 Bad Request when the payload is {}",
            error_message
        );
    }
}
