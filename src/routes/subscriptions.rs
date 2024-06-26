use actix_web::{post, web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
struct SubscriptionFormData {
    name: String,
    email: String,
}

#[post("/subscriptions")]
pub async fn subscriptions(form_data: web::Form<SubscriptionFormData>) -> HttpResponse {
    let email_regex = regex::Regex::new(r"^[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,4}$").unwrap();

    if !email_regex.is_match(form_data.email.trim()) {
        return HttpResponse::BadRequest().finish();
    };

    if form_data.name.trim().is_empty() {
        return HttpResponse::BadRequest().finish();
    }
    HttpResponse::Ok().finish()
}
