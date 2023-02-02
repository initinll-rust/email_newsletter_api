use actix_web::{HttpResponse, web};
use sqlx::{PgPool};
use chrono::Utc;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}

pub async fn subscribe(
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>) -> HttpResponse {
    // random unique identifier
    let request_id = Uuid::new_v4();   
    
    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );

    let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!(
        "Saving new subscriber details in the database."
    );

    // logging
    tracing::info!("Request_Id {} - Adding '{}' '{}' as a new subscriber.", request_id, form.email, form.name);
    tracing::info!("Request_Id {} - Saving new subscriber details in the database", request_id);

    let result = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .instrument(query_span)
    .await;

    match result
    {
        Ok(_) => {
            // logging Success !
            tracing::info!("Request_Id {} - New subscriber details have been saved", request_id);
        
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            // logging Error !
            tracing::error!("Request_Id {} - Failed to execute query: {:?}", request_id, e);
        
            HttpResponse::InternalServerError().finish()
        }
    }    
}