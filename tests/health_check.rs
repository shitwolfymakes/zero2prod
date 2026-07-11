//! tests/health_check.rs

use std::net::TcpListener;

use reqwest::Client;

// Launch our application in the background ~somehow~
// No .await call, therefore no need for `spawn_app` to be async now.
// We are also running tests, so it's not worth it to propagate errors
// if we fail to perform the required setup we can just panic and crash
// all the things
/// Spin up an instance of out application
/// and return it's address (i.e., http://localhost:XXXX)
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind to random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    // we return the application address to the caller!
    return format!("http://127.0.0.1:{}", port);
}

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
    .get(format!("{}/health_check", &address))
    .send()
    .await
    .expect("Failed to execute request");
    
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    // act
    let body = "name=le%20guin&email=ursula_k_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16())
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20gion", "missing the email"),
        ("email=ursula_k_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];

    for (invalid_body, error_message) in test_cases {
        // act
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // additional customized error message on test failure
            "The api did not fail with a 400 request when the payload was {}",
            error_message
        );
    }
}
