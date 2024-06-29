use std::net::TcpListener;

use sqlx::Pool;
use sqlx::Postgres;
use uuid::Uuid;
use zero2prod::configuration::configure_database;
use zero2prod::telemetry;
use zero2prod::{configuration, run};

struct TestConfig {
    server_url: String,
    db_pool: Pool<Postgres>,
}

#[derive(Debug)]
struct TestCase<T> {
    case: T,
    err_msg: String,
}

async fn spawn_app() -> Result<TestConfig, std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let subscriber = telemetry::get_subscriber("into".into(), "debug".into());
    telemetry::init_subscriber(subscriber);

    let port = listener.local_addr().unwrap().port().to_string();

    let mut server_settings =
        configuration::get_configuration().expect("failed to load server settings");

    server_settings.database.database_name = Uuid::new_v4().to_string();

    let db_pool = configure_database(&server_settings.database).await;

    let server = run(listener, db_pool.clone())?;
    let _ = tokio::spawn(server);

    Ok(TestConfig {
        server_url: format!("http://127.0.0.1:{port}"),
        db_pool,
    })
}

#[tokio::test]
async fn health_check_works() {
    let TestConfig {
        server_url,
        db_pool: _,
    } = spawn_app().await.expect("Failed to spawn app.");

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", server_url))
        .send()
        .await
        .expect("failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // arrange
    let TestConfig {
        server_url,
        db_pool,
    } = spawn_app().await.expect("Failed to spawn app.");

    let client = reqwest::Client::new();

    let test_cases = [
        TestCase {
            case: [("email", "bechir@gmail.com"), ("name", "bayi")],
            err_msg: "valid".to_owned(),
        },
        TestCase {
            case: [("email", "bechirmakhlouf2020@gmail.com"), ("name", "noyi")],
            err_msg: "valid".to_owned(),
        },
        TestCase {
            case: [("email", "bechirmakhlouf123@gmail.com"), ("name", "noya")],
            err_msg: "valid".to_owned(),
        },
        TestCase {
            case: [("email", "bechirmakhlouf11123@gmail.com"), ("name", "yeee")],
            err_msg: "valid".to_owned(),
        },
    ];

    // act
    for test_case in test_cases {
        let response = client
            .post(format!("{}/subscriptions", server_url))
            .form(&test_case.case)
            .send()
            .await
            .expect("failed to execute request");

        let retrieved_row = sqlx::query!(
            "SELECT name, email FROM subscriptions where email = $1",
            test_case.case[0].1
        )
        .fetch_one(&db_pool)
        .await
        .expect("failed to retreive subscription row");

        assert_eq!(retrieved_row.email, test_case.case[0].1);
        assert_eq!(retrieved_row.name, test_case.case[1].1);

        // assert
        assert_eq!(
            200,
            response.status().as_u16(),
            "{}",
            format!(
                "The API did not succeed with 200 OK when the payload was  {:?}\n",
                test_case
            )
        );
    }
}

#[tokio::test]
async fn subscribe_return_400_for_invalid_form_data() {
    // arrange
    let TestConfig {
        server_url,
        db_pool: _,
    } = spawn_app().await.expect("Failed to spawn app.");
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
            .post(format!("{}/subscriptions", server_url))
            .form(&test_case.case)
            .send()
            .await
            .expect("failed to execute request");

        // assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            test_case.err_msg
        );
    }
}
