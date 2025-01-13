use anyhow::anyhow;
use axum::{
    body::Body,
    extract::{Request, State},
    http::{self, Response},
    middleware::Next,
};
use derive_more::Display;

use crate::errors;

use super::AppState;

#[derive(Debug, Display)]
pub enum AuthError {
    #[display(fmt = "decode")]
    Decode(anyhow::Error),
    #[display(fmt = "not_found")]
    NotFound(String),
    #[display(fmt = "no_token")]
    NoToken(),
}

pub async fn middleware(
    State(AppState { authentication, .. }): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, errors::ServerError> {
    let mut auth_header = req
        .headers_mut()
        .get(http::header::AUTHORIZATION)
        .ok_or_else(|| errors::ServerError::Unauthenticated)?
        .to_str()
        .map_err(|err| errors::ServerError::UnauthenticatedReason(anyhow!(err)))?
        .split_whitespace();
    let (_bearer, token) = (auth_header.next(), auth_header.next());
    let token = token.ok_or_else(|| errors::ServerError::Unauthenticated)?;
    let _claims = authentication
        .validate_token(token.to_string())
        .await
        .map_err(|err| errors::ServerError::UnauthenticatedReason(anyhow!(err)))?;

    // let auth_header = match auth_header {
    //     Some(header) => header
    //         .to_str()
    //         .map_err(|err| AuthError::Decode(anyhow!(err)))?,
    //     None => return Err(AuthError::NoToken()),
    // };
    // let mut header = auth_header.split_whitespace();
    // let (bearer, token) = (header.next(), header.next());
    // let token_data = match decode_jwt(token.unwrap().to_string()) {
    //     Ok(data) => data,
    //     Err(err) => return Err(AuthError::Decode(err)),
    // };
    Ok(next.run(req).await)
}
