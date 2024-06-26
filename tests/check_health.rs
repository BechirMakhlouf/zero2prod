use std::net::TcpListener;

use zero2prod::run;

struct TestCase<T> {
    case: T,
    err_msg: String,
}

fn spawn_app() -> Result<String, std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr().unwrap().port().to_string();
    let server = run(listener)?;
    let _ = tokio::spawn(server);
    println!("port: {port}");
    Ok(format!("http://127.0.0.1:{}", port))
}

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

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // arrange
    let server_addr = spawn_app().expect("Failed to spawn app.");
    let client = reqwest::Client::new();

    let test_cases = [
        TestCase {
            case: vec![("email", "bechir@gmail.com"), ("name", "bayi")],
            err_msg: "with invalid email".to_owned(),
        },
        TestCase {
            case: vec![("email", "bechirmakhlouf2020@gmail.com"), ("name", "noyi")],
            err_msg: "with no email".to_owned(),
        },
        TestCase {
            case: vec![("email", "bechirmakhlouf123@gmail.com"), ("name", "noya")],
            err_msg: "with no name".to_owned(),
        },
        TestCase {
            case: vec![("email", "bechirmakhlouf11123@gmail.com"), ("name", "yeee")],
            err_msg: "with nothing provided".to_owned(),
        },
    ];

    // act
    for test_case in test_cases {
        let response = client
            .post(format!("{}/subscriptions", server_addr))
            .form(&test_case.case)
            .send()
            .await
            .expect("failed to execute request");

        // assert
        assert_eq!(
            200,
            response.status().as_u16(),
            "The API did not succeed with 200 OK when the payload was {}.",
            test_case.err_msg
        );
    }
}

#[tokio::test]
async fn subscribe_return_400_for_invalid_form_data() {
    // arrange
    let server_addr = spawn_app().expect("Failed to spawn app.");
    let client = reqwest::Client::new();

    let test_cases = [
        TestCase {
            case: vec![("email", "invalid_email"), ("name", "bayi")],
            err_msg: "with invalid email".to_owned(),
        },
        TestCase {
            case: vec![("name", "bayi")],
            err_msg: "with no email".to_owned(),
        },
        TestCase {
            case: vec![("email", "invalid_email")],
            err_msg: "with no name".to_owned(),
        },
        TestCase {
            case: vec![],
            err_msg: "with nothing provided".to_owned(),
        },
    ];

    // act
    for test_case in test_cases {
        let response = client
            .post(format!("{}/subscriptions", server_addr))
            .form(&test_case.case)
            .send()
            .await
            .expect("failed to execute request");

        // act
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            test_case.err_msg
        );
    }
}
