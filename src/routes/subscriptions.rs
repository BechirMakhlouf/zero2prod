use actix_web::{post, web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use sqlx::{pool, PgConnection, Pool, Postgres};
use uuid::Uuid;

#[derive(Deserialize)]
struct SubscriptionFormData {
    name: String,
    email: String,
}

#[post("/subscriptions")]
pub async fn subscriptions(
    form_data: web::Form<SubscriptionFormData>,
    db_pool: web::Data<Pool<Postgres>>,
) -> HttpResponse {
    let request_id = Uuid::new_v4();

    tracing::info!(
        "request_id: {} - Adding '{} {}' as a subscriber.",
        request_id,
        form_data.name.trim(),
        form_data.email.trim()
    );
    let email_regex = regex::Regex::new(r"^[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,4}$").unwrap();

    if !email_regex.is_match(form_data.email.trim()) {
        return HttpResponse::BadRequest().finish();
    };
    if form_data.name.trim().is_empty() {
        return HttpResponse::BadRequest().finish();
    }

    tracing::info!("requestid: {request_id} - saving new subscriber to the database.");
    match sqlx::query!(
        r#"
      INSERT INTO subscriptions (id, email, name, subscribed_at)
      VALUES ($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        form_data.email.trim(),
        form_data.name.trim(),
        Utc::now()
    )
    .execute(db_pool.get_ref())
    .await
    {
        Ok(_) => {
            tracing::info!("request_id: {request_id} - new subscriber details have been saved.");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("request_id {request_id} - Failed to execute query: {e:?}");
            HttpResponse::BadRequest().finish()
        }
    }
}
