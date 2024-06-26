mod configuration;
mod routes;
use std::net::TcpListener;
use zero2prod::run;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind to a random port");
    let _configuration = configuration::get_configuration().unwrap();

    println!("running on port: {}", listener.local_addr().unwrap().port());
    let _ = run(listener)?.await;
    Ok(())
}
