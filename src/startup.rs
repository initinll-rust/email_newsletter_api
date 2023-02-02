use std::net::TcpListener;
use sqlx::{PgPool};
use actix_web::{dev::Server, HttpServer, App, web};
use tracing_actix_web::TracingLogger;

use crate::routes::*;

pub fn run_app(listener: TcpListener, db_pool: PgPool) -> Result<Server,std::io::Error> {
    // Wrap the db_pool in a smart pointer
    let db_pool = web::Data::new(db_pool);
    // Capture `db_pool` from the surrounding environment
    let server = HttpServer::new(move || {
            App::new()
                // Middleware are added using the `wrap` method on `App`
                .wrap(TracingLogger::default())
                .route("/health_check", web::get().to(health_check))
                .route("/subscriptions", web::post().to(subscribe))
                // Get a pointer copy and attach it to the application state
                .app_data(db_pool.clone())
        })
        .listen(listener)?
        .run();
        // No await here!
    Ok(server)
}