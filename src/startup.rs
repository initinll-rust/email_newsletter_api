use std::net::TcpListener;

use actix_web::{dev::Server, HttpServer, App, web};

use crate::routes::*;

pub fn run_app(listener: TcpListener) -> Result<Server,std::io::Error> {
    let server = HttpServer::new(|| {
            App::new()
                .route("/health_check", web::get().to(health_check))
                .route("/subscriptions", web::post().to(subscribe))
        })
        .listen(listener)?
        .run();
        // No await here!
    Ok(server)
}