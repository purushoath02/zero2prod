use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum::{routing::get, Router};
use std::net::SocketAddr;
use std::net::{IpAddr, Ipv4Addr};
use tokio::net::TcpListener;
use zero2pod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let address = listener.local_addr().unwrap().to_string();
    println!("{:?}", address);
    let _ = run(listener).await?;
    Ok(())
}
// async fn health_check() -> Response {
//     Response::builder()
//         .status(StatusCode::OK)
//         .header("x-foo", "header")
//         .body(Body::from("Health is okay"))
//         .unwrap()
// }
