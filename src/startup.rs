use std::net::TcpListener;
use sqlx::PgConnection;

use actix_web::{dev::Server, HttpServer, App, web};

use crate::routes::*;

pub fn run_app(listener: TcpListener, connection: PgConnection) -> Result<Server,std::io::Error> {
    // Wrap the connection in a smart pointer
    let connection = web::Data::new(connection);
    // Capture `connection` from the surrounding environment
    let server = HttpServer::new(move || {
            App::new()
                .route("/health_check", web::get().to(health_check))
                .route("/subscriptions", web::post().to(subscribe))
                // Get a pointer copy and attach it to the application state
                .app_data(connection.clone())
        })
        .listen(listener)?
        .run();
        // No await here!
    Ok(server)
}