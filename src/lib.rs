pub mod configuration;
mod routes;
use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::{Pool, Postgres};

pub fn run(listener: TcpListener, db_pool: Pool<Postgres>) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(connection.clone())
            .service(routes::health_check)
            .service(routes::subscriptions)
    })
    .listen(listener)?
    .run();
    Ok(server)
}
