use crate::ctx::Ctx;
use crate::model::ModelController;
use crate::{web::AUTH_TOKEN, ApiError, Result};
use axum::extract::State;
use axum::http::request::Parts;
use axum::{
    body::Body,
    extract::{FromRequestParts, Request},
    middleware::Next,
    response::Response,
};
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};

pub async fn mw_require_auth(ctx: Result<Ctx>, req: Request<Body>, next: Next) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver(
    _mc: State<ModelController>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    println!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // Compute Result<Ctx>.
    let result_ctx = match auth_token
        .ok_or(ApiError::AuthFailedNoAuthTokenInCookie)
        .and_then(parse_token)
    {
        Ok((user_id, _exp, _sign)) => {
            // TODO: Token components validations.
            Ok(Ctx::new(user_id))
        },
        Err(e) => Err(e),
    };

    // Remove the cookie if something went wrong
    if result_ctx.is_err() && !matches!(result_ctx, Err(ApiError::AuthFailedNoAuthTokenInCookie)) {
        cookies.remove(Cookie::from(AUTH_TOKEN))
    }

    // Store the ctx_result in the request extension.
    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

/// Parse a token of format `user-[user-id].[expiration].[signature]`
/// Returns (user_id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
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

        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(ApiError::Unauthorized(
                "Cookie doesn't have context.".to_string(),
            ))?
            .clone()
    }
}
