use anyhow::anyhow;
use axum::{body::Body, extract::Request, extract::State, http::Response, middleware::Next};

use crate::{authorization::User, errors};

use super::AppState;

pub async fn is_user_action_allowed(
    State(AppState { authorization, .. }): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response<Body>, errors::ServerError> {
    let is_authorized = authorization
        .is_action_allowed(
            User {
                user_id: "random".to_owned(),
                role: "random".to_owned(),
            },
            "list_users".to_owned(),
        )
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    if is_authorized {
        return Ok(next.run(req).await);
    }
    Err(errors::ServerError::Unauthorized)
}
