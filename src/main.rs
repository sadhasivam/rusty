use axum::http;
use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port  = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let socket_addr: SocketAddr = addr.parse().expect("Unable to parse socket address");

   

    let app = Router::new().route("/", get(health));
    let listener = tokio::net::TcpListener::bind(&socket_addr).await.unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();

    Ok(())

}

async fn health() -> http::StatusCode {
    http::StatusCode::OK
}