use std::net::TcpListener;

use email_newsletter_api::startup::run_app;
use email_newsletter_api::configuration::get_configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(&address).expect("Failed to bind random port");
    println!("server listening at - {}", &address);
    run_app(listener)?.await
}
