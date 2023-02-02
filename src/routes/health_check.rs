use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse {
    // logging
    tracing::info!("checking app health");
    HttpResponse::Ok().finish()
}