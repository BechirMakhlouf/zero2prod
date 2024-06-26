pub mod configuration;
mod routes;
use std::net::TcpListener;

use actix_web::{dev::Server, App, HttpServer};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(routes::health_check)
            .service(routes::subscriptions)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
