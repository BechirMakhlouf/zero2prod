mod configuration;
mod routes;
mod telemetry;
use configuration::Settings;
use std::net::TcpListener;
use telemetry::init_subscriber;
use zero2prod::run;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = telemetry::get_subscriber("into".into(), "debug".into());
    init_subscriber(subscriber);

    let Settings {
        application,
        database: _,
    } = configuration::get_configuration().unwrap();

    let server_url = format!("{}:{}", application.host, application.port);
    let listener = TcpListener::bind(server_url).unwrap();

    let connection_string = configuration::get_configuration()
        .expect("failed to get configuration")
        .database
        .connection_string();

    let db_pool = sqlx::postgres::PgPool::connect_lazy(&connection_string)
        .expect("failed to connect to postgres database");

    println!("running on port: {}", listener.local_addr().unwrap().port());
    let _ = run(listener, db_pool)?.await;

    Ok(())
}
