use crate::{web::AUTH_TOKEN, ApiError, Result};
use axum::{body::Body, extract::Request, middleware::Next, response::Response};
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

pub async fn mw_require_auth(cookies: Cookies, req: Request<Body>, next: Next) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");
    let auth_token = cookies
        .get(AUTH_TOKEN)
        .map(|c| c.value().to_string())
        .ok_or(ApiError::Unauthorized(
            "Authentication Failed, incorrect credentials.".to_string(),
        ))?;

    // Parse Token
    let (user_id, expiration, signature) = parse_token(auth_token).await?;

    // Some validation on data of token obtained after parsing
    // TODO

    Ok(next.run(req).await)
}

/// Parse a token of format `user-[user-id].[expiration].[signature]`
/// Returns (user_id, expiration, signature)
pub async fn parse_token(token: String) -> Result<(u64, String, String)> {
    let common_err =
        ApiError::Unauthorized("Authentication Failed, wrong auth-token format.".to_string());
    let (_whole, user_id, expiration, signature) =
        regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token).ok_or(common_err.clone())?;

    let user_id: u64 = user_id.parse().map_err(|_| common_err)?;

    Ok((user_id, expiration.to_string(), signature.to_string()))
}
