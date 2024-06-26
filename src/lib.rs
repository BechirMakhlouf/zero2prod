use std::net::TcpListener;

use actix_web::{dev::Server, get, post, web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct SubscriptionFormData {
    name: String,
    email: String,
}

#[post("/subscriptions")]
async fn subscriptions(form_data: web::Form<SubscriptionFormData>) -> HttpResponse {
    let email_regex = regex::Regex::new(r"^[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,4}$").unwrap();

    if !email_regex.is_match(form_data.email.trim()) {
        return HttpResponse::BadRequest().finish();
    };

    if form_data.name.trim().is_empty() {
        return HttpResponse::BadRequest().finish();
    }
    HttpResponse::Ok().finish()
}

#[get("/health_check")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health_check).service(subscriptions))
        .listen(listener)?
        .run();

    Ok(server)
}
