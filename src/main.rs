use std::net::SocketAddr;

use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::{get, get_service},
    Router,
};
use axum_youtube_code_along::error::MainError;
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;

#[derive(Debug, Serialize, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), MainError> {
    let routes_all = Router::new()
        .merge(routes_hello())
        .fallback_service(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> Listening on {addr}\n");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|err| MainError::new(err.to_string()))?;

    axum::serve(listener, routes_all)
        .await
        .map_err(|err| MainError::new(err.to_string()))?;

    Ok(())
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(hello_handler))
        .route("/hello2/{name}", get(hello_handler_2))
}

async fn hello_handler(Query(q_params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - hello_handler - {q_params:?}", "HANDLER");

    let name = q_params.name.as_deref().unwrap_or("World!!!");
    Html(format!("Hello <strong>{name}</strong>"))
}

async fn hello_handler_2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - hello_handler - {name:?}", "HANDLER");
    Html(format!("Hello <strong>{name}</strong>"))
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
