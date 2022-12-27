use std::net::TcpListener;


#[tokio::test]
async fn health_check() {
    // Arrange
    let address = spawn_app();
    let endpoint = format!("{}/health_check", address);

    let client = reqwest::Client::new();

    // Act
    let response = client
            .get(endpoint)
            .send()
            .await
            .expect("Failed to execute request.");
    
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

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