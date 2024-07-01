pub mod configuration;
mod routes;
pub mod telemetry;
use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::{Pool, Postgres};
use tracing_actix_web::TracingLogger;

pub fn run(listener: TcpListener, db_pool: Pool<Postgres>) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(db_pool);

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
