pub mod configuration;
mod routes;
pub mod telemetry;
use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::{Pool, Postgres};
use tracing_actix_web::TracingLogger;

pub fn run(listener: TcpListener, db_pool: Pool<Postgres>) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(db_pool);

    // let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    // let formatting_layer = BunyanFormattingLayer::new(
    //     "zero2prod".into(),
    //     // Output the formatted spans to stdout.
    //     std::io::stdout,
    // );
    // let subscriber = Registry::default()
    //     .with(env_filter)
    //     .with(JsonStorageLayer)
    //     .with(formatting_layer);
    // set_global_default(subscriber).expect("Failed to set subscriber");

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(connection.clone())
            .service(routes::health_check)
            .service(routes::subscriptions)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
