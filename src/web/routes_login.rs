use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

use crate::{web::AUTH_TOKEN, ApiError, Result};

#[derive(Debug, Deserialize, Serialize)]
struct LoginPayload {
    username: String,
    password: String,
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    // TODO: Real db logic here
    if payload.username != "demo1" || payload.password != "welcome" {
        return Err(ApiError::Unauthorized(
            "Login Failed. Incorrect credentials.".to_string(),
        ));
    }

    // FIX_ME: Add real token generation
    cookies.add(Cookie::new(AUTH_TOKEN, "DDDD-user-1.exp.signature"));

    // Create the success body
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));
    Ok(body)
}

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}
