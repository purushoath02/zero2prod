use std::fmt::format;
use std::net::{IpAddr, Ipv4Addr};

use axum::extract::Form;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::serve::Serve;
use axum::Error;
use axum::{body::Body, http::StatusCode, response::Response};
use axum::{routing::get, Router};
//use reqwest::Response;
use tokio::net::TcpListener;

#[derive(serde::Deserialize)]
pub struct userdata {
    username: String,
    email: String,
}

pub async fn run(listener: TcpListener) -> std::io::Result<()> {
    // let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0);

    //let listener = tokio::net::TcpListener::bind(address).await?
    let address = listener.local_addr().unwrap();
    println!("Server is active @ {}", address);
    axum::serve(listener, app()?).await.unwrap();
    Ok(())
}
async fn health_check() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header("x-foo", "header")
        .body(Body::from("Health is okay"))
        .unwrap()
}
async fn subscribe() -> impl IntoResponse {
    println!("subscribed");
    StatusCode::OK
}
pub fn app() -> std::io::Result<Router> {
    let app = Router::new()
        .route("/health_check", get(|| health_check()))
        .route("/subscribe", post(|| subscribe()));
    Ok(app)
}

pub fn index(Form(form): Form<userdata>) -> String {
    format!("welcome {}", form.username)
}
