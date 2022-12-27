use std::net::TcpListener;

fn spawn_app() -> String {
    // Port 0 is special-cased at the OS level: trying to bind port 0 will trigger an OS scan for an availableport which will then be bound to the application.
    let listener = TcpListener::bind("127.0.0.1:0")
            .expect("Failed to bind random port");

    // We retrive the port assigned to us by the os
    let port = listener.local_addr().unwrap().port();

    let server = email_newsletter_api::run_app(listener)
            .expect("Failed to bind address");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}


#[tokio::test]
async fn health_check() {
    // Arrange
    let app_address = spawn_app();
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
    let app_address = spawn_app();
    let subscriptions_endpoint = format!("{}/subscriptions", app_address);
    let client = reqwest::Client::new();

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
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app_address = spawn_app();
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