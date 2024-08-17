use axum::response::Html;
use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new().route("/", get(|| health_check()));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn health_check() -> Html<&'static str> {
    Html("<h1>Hello, world!</h1>")
}
