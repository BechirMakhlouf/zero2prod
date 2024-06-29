pub mod configuration;
mod routes;
use std::net::TcpListener;

use actix_web::{dev::Server, middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use sqlx::{Pool, Postgres};

pub fn run(listener: TcpListener, db_pool: Pool<Postgres>) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(db_pool);

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(connection.clone())
            .service(routes::health_check)
            .service(routes::subscriptions)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
