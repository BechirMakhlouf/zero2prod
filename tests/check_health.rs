use std::net::TcpListener;

use zero2prod::run;

#[tokio::test]
async fn health_check_works() {
    let server_addr = spawn_app().expect("Failed to spawn app.");
    let client = reqwest::Client::new();

    println!("server addr: {}", &server_addr);

    let response = client
        .get(format!("{}/health_check", server_addr))
        .send()
        .await
        .expect("failed to execute request");

    println!("status: {}", response.status());

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

fn spawn_app() -> Result<String, std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr().unwrap().port().to_string();
    let server = run(listener)?;
    let _ = tokio::spawn(server);
    println!("port: {port}");
    Ok(format!("http://127.0.0.1:{}", port))
}

// #[cfg(test)]
// mod tests {
//     use crate::health_check;
//
//     #[tokio::test]
//     async fn health_check_succeeds() {
//         let response = health_check().await;
//         assert!(response.status().is_success())
//     }
// }
