use actix_web::{HttpResponse, web};
use sqlx::{PgPool};
use chrono::Utc;
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

    // logging
    log::info!("Request_Id {} - Adding '{}' '{}' as a new subscriber.", request_id, form.email, form.name);
    log::info!("Request_Id {} - Saving new subscriber details in the database", request_id);

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
    .await;

    match result
    {
        Ok(_) => {
            // logging Success !
            log::info!("Request_Id {} - New subscriber details have been saved", request_id);
        
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            // logging Error !
            log::error!("Request_Id {} - Failed to execute query: {:?}", request_id, e);
        
            HttpResponse::InternalServerError().finish()
        }
    }    
}