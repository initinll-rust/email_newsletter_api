use std::net::TcpListener;
use sqlx::{PgConnection, Connection};

use email_newsletter_api::startup::run_app;
use email_newsletter_api::configuration::get_configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection = PgConnection::connect(&configuration.database.connection_string())
            .await
            .expect("Failed to connect to Postgres.");

    let host = configuration.database.host;
    let port = configuration.application_port;
    let address = format!("{}:{}",host, port);

    let listener = TcpListener::bind(&address)
            .expect("Failed to bind random port");
    println!("server listening at - {}", &address);
    
    run_app(listener, connection)?.await
}
