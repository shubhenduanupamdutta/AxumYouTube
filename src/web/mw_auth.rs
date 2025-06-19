use crate::{web::AUTH_TOKEN, ApiError, Result};
use axum::{body::Body, extract::Request, middleware::Next, response::Response};
use tower_cookies::Cookies;

pub async fn mw_require_auth(cookies: Cookies, req: Request<Body>, next: Next) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");
    let _auth_token = cookies
        .get(AUTH_TOKEN)
        .map(|c| c.value().to_string())
        .ok_or(ApiError::Unauthorized(
            "Authentication Failed, incorrect credentials.".to_string(),
        ))?;

    Ok(next.run(req).await)
}
