use std::net::TcpListener;

use email_newsletter_api::run_app;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let listener = TcpListener::bind("127.0.0.1:8000")
            .expect("Failed to bind random port");

    run_app(listener)?.await
}
