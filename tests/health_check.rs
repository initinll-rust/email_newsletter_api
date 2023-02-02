use std::net::TcpListener;
use sqlx::{PgPool, PgConnection, Connection, Executor};
use uuid::Uuid;
use once_cell::sync::Lazy;
use secrecy::ExposeSecret;

use email_newsletter_api::configuration::{get_configuration, DatabaseSettings};
use email_newsletter_api::telemetry::{get_subscriber, init_subscriber};
use email_newsletter_api::startup::run_app;

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool
}

async fn spawn_test_app() -> TestApp {
    
    Lazy::force(&TRACING);

    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = Uuid::new_v4().to_string();

    let connection_pool = configure_database(&configuration.database).await;

    let host = configuration.database.host;
    // Port 0 is special-cased at the OS level: trying to bind port 0 will trigger an OS scan for an availableport which will then be bound to the application.    
    let address = format!("{}:0",host);

    let listener = TcpListener::bind(&address)
            .expect("Failed to bind random port");

    // We retrive the port assigned to us by the os
    let port = listener.local_addr().unwrap().port();
    let new_address = format!("http://{}:{}", host, port);

    let server = run_app(listener, connection_pool.clone())
            .expect("Failed to bind address");

    let _ = tokio::spawn(server);

    TestApp {
        address: new_address,
        db_pool: connection_pool
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect(&config.connection_string_without_db().expose_secret())
                .await
                .expect("Failed to connect to Postgres.");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect(&config.connection_string().expose_secret())
            .await
            .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}


#[tokio::test]
async fn health_check() {
    // Arrange
    let test_app = spawn_test_app().await;
    let health_check_endpoint = format!("{}/health_check", test_app.address);
    let client = reqwest::Client::new();

    // Act
    let response = client
            .get(health_check_endpoint)
            .send()
            .await
            .expect("Failed to execute request.");
    
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let test_app = spawn_test_app().await;    
    let client = reqwest::Client::new();
    let subscriptions_endpoint = format!("{}/subscriptions", test_app.address);    

    // Act
    let body = "name=test%20name&email=testname%40gmail.com";
    let response = client
            .post(subscriptions_endpoint)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
            .fetch_one(&test_app.db_pool)
            .await
            .expect("Failed to fetch saved subscription.");
    
    assert_eq!(saved.email, "testname@gmail.com");
    assert_eq!(saved.name, "test name");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let test_app = spawn_test_app().await;    
    let subscriptions_endpoint = format!("{}/subscriptions", test_app.address);
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin","missing the email"),
        ("email=ursula_le_guin%40gmail.com","missing the name"),
        ("","missing both name and email")
    ];

    // Act
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&subscriptions_endpoint)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(
            400, 
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}