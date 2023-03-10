use std::net::TcpListener;
use sqlx::PgPool;
use secrecy::ExposeSecret;

use email_newsletter_api::startup::run_app;
use email_newsletter_api::configuration::get_configuration;
use email_newsletter_api::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {           

        let subscriber = get_subscriber("zero2prod".into(),"info".into(), std::io::stdout);
        init_subscriber(subscriber);

        let configuration = get_configuration().expect("Failed to read configuration.");
        
        let connection_pool = PgPool::connect_lazy(
                &configuration.database.connection_string().expose_secret()
        ).expect("Failed to connect to Postgres.");

        let host = configuration.database.host;
        let port = configuration.application_port;
        let address = format!("{}:{}",host, port);

        let listener = TcpListener::bind(&address)
                .expect("Failed to bind random port");
        println!("server listening at - {}", &address);

        run_app(listener, connection_pool)?.await
}