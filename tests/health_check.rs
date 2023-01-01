use std::net::TcpListener;
use email_newsletter_api::configuration::get_configuration;
use sqlx::{PgConnection, Connection};

async fn spawn_app() -> String {
    let configuration = get_configuration().expect("Failed to read configuration.");
    
    let connection = PgConnection::connect(&configuration.database.connection_string())
            .await
            .expect("Failed to connect to Postgres.");

    let host = configuration.database.host;
    // Port 0 is special-cased at the OS level: trying to bind port 0 will trigger an OS scan for an availableport which will then be bound to the application.    
    let address = format!("{}:0",host);

    let listener = TcpListener::bind(&address)
            .expect("Failed to bind random port");

    // We retrive the port assigned to us by the os
    let port = listener.local_addr().unwrap().port();

    let server = email_newsletter_api::startup::run_app(listener, connection)
            .expect("Failed to bind address");

    let _ = tokio::spawn(server);

    format!("http://{}:{}", host, port)
}


#[tokio::test]
async fn health_check() {
    // Arrange
    let app_address = spawn_app().await;
    let health_check_endpoint = format!("{}/health_check", app_address);
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
    let app_address = spawn_app().await;
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let mut connection = PgConnection::connect(&connection_string)
            .await
            .expect("Failed to connect to postgres.");
    
    let client = reqwest::Client::new();
    let subscriptions_endpoint = format!("{}/subscriptions", app_address);    

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
            .fetch_one(&mut connection)
            .await
            .expect("Failed to fetch saved subscription.");
    
    assert_eq!(saved.email, "testname@gmail.com");
    assert_eq!(saved.name, "test name");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app_address = spawn_app().await;
    let subscriptions_endpoint = format!("{}/subscriptions", app_address);
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