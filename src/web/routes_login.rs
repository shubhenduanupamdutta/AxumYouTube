use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{ApiError, Result};

#[derive(Debug, Deserialize, Serialize)]
struct LoginPayload {
    username: String,
    password: String,
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->>{:<12} - api_login", "HANDLER");

    // TODO: Real db logic here
    if payload.username != "demo1" || payload.password != "welcome" {
        return Err(ApiError::Unauthorized(
            "Login Failed. Incorrect credentials.".to_string(),
        ));
    }

    // TODO: Set Cookie

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
