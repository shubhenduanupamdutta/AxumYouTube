use std::net::SocketAddr;

use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use axum_youtube_code_along::error::MainError;

#[tokio::main]
async fn main() -> Result<(), MainError> {
    let routes_hello = Router::new().route("/hello", get(hello_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> Listening on {addr}\n");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|err| MainError::new(err.to_string()))?;

    axum::serve(listener, routes_hello)
        .await
        .map_err(|err| MainError::new(err.to_string()))?;

    Ok(())
}

async fn hello_handler() -> impl IntoResponse {
    println!("->> {:<12} - hello_handler", "HANDLER");

    Html("Hello <strong>World!!!</strong>")
}
