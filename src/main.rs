use std::net::SocketAddr;

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    middleware,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
mod error;
pub mod ctx;
pub mod model;
pub mod web;
pub use error::{ApiError, Result};
use serde::{Deserialize, Serialize};
use tower_cookies::CookieManagerLayer;

use crate::{model::ModelController, web::mw_auth::mw_require_auth};

#[derive(Debug, Serialize, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;

    let route_apis =
        web::routes_tickets::routes(mc.clone()).route_layer(middleware::from_fn(mw_require_auth));

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", route_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback(handler_404);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> Listening on {addr}\n");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|err| ApiError::new(err.to_string()))?;

    axum::serve(listener, routes_all)
        .await
        .map_err(|err| ApiError::new(err.to_string()))?;

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

// Method 1: Embed the HTML directly in the binary (recommended)
const ERROR_404_HTML: &str = include_str!("./error/404.html");

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Html(ERROR_404_HTML))
}

/// This will allow and empty line between two req/response cycle for easy understanding
async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    println!();
    res
}
