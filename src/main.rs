mod configuration;
mod routes;
mod telemetry;
use std::net::TcpListener;
use telemetry::init_subscriber;
use zero2prod::run;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = telemetry::get_subscriber("into".into(), "debug".into());
    init_subscriber(subscriber);

    let configuration = configuration::get_configuration().unwrap();
    let server_url = format!("127.0.0.1:{}", &configuration.application_port);
    let listener = TcpListener::bind(server_url).unwrap();
    let connection_string = configuration::get_configuration()
        .expect("failed to get configuration")
        .database
        .connection_string();
    let db_pool = sqlx::postgres::PgPool::connect(&connection_string)
        .await
        .expect("failed to connect to postgres database");

    println!("running on port: {}", listener.local_addr().unwrap().port());
    let _ = run(listener, db_pool)?.await;

    Ok(())
}
