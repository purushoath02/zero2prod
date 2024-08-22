//use reqwest::{self, Request};
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
use zero2pod::{app, run}; // Assuming this is your server module

#[tokio::test]
async fn health_check_test() {
    // Create an instance of the app
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind TCP listener");

    // Get the address and format it into a URL
    let address = listener.local_addr().expect("Failed to get local address");
    let address = format!("http://{}", address);

    // Spawn the server
    tokio::spawn(async move {
        // Run the server only once
        zero2pod::run(listener).await.expect("Server failed");
    });

    // Wait for the server to be ready (adjust duration as needed)
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Send a request and check response
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    // Ensure the server task completes
}
