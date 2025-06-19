use crate::ctx::Ctx;
use crate::{web::AUTH_TOKEN, ApiError, Result};
use axum::http::request::Parts;
use axum::RequestPartsExt;
use axum::{
    body::Body,
    extract::{FromRequestParts, Request},
    middleware::Next,
    response::Response,
};
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

pub async fn mw_require_auth(ctx: Result<Ctx>, req: Request<Body>, next: Next) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");

    ctx?;

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

/// Context Extractor
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    #[doc = " If the extractor fails it\'ll use this \"rejection\" type. A rejection is"]
    #[doc = " a kind of error that can be converted into a response."]
    type Rejection = ApiError;

    #[doc = " Perform the extraction."]
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<12} - Ctx", "EXTRACTOR");

        let cookies = parts.extract::<Cookies>().await.map_err(|_| {
            ApiError::InternalServerError("Failing to extract cookie details.".to_string())
        })?;

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

        Ok(Ctx::new(user_id))
    }
}
