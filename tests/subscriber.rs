use axum::{body::Body, response::IntoResponse, Router};
use axum_test_helper::TestClient;
use http::{self, response, StatusCode};
use hyper::service;
use std::mem::zeroed;

use reqwest;
use reqwest::Request;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use tower::{Service, ServiceExt};
use zero2pod;
use zero2pod::{app, run};

#[tokio::test]
async fn subsribe_200_status_ok() {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind TCP listener");

    // Get the address and format it into a URL
    let address = listener.local_addr().expect("Failed to get local address");
    let address = format!("http://{}", address);
    tokio::spawn(async move {
        // Run the server only once
        zero2pod::run(listener).await.expect("Server failed");
    });
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(format!("{}/subscribe", address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn subscribe_400_not_ok() {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind TCP listener");

    // Get the address and format it into a URL
    let address = listener.local_addr().expect("Failed to get local address");
    let address = format!("http://{}", address);
    tokio::spawn(async move {
        // Run the server only once
        zero2pod::run(listener).await.expect("Server failed");
    });
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscribe", address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
        // Assert
        assert_eq!(
            StatusCode::BAD_REQUEST.as_str(),
            response.status().as_str(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
