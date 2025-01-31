use anyhow::anyhow;
use axum::{
    body::Body,
    extract::{Request, State},
    http::Response,
    middleware::Next,
    Extension,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    authentication::Claims,
    authorization::User as AuthzUser,
    errors,
    models::{self, user::Entity as User},
};

use super::AppState;

pub async fn can_list_users(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    req: Request,
    next: Next,
) -> Result<Response<Body>, errors::ServerError> {
    let conn = &*state.conn.clone();
    let authorization = state.authorization.clone();

    let user = User::find()
        .filter(models::user::Column::Auth0Id.eq(claims.sub.to_owned()))
        .one(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?
        .ok_or(errors::ServerError::Unauthorized)?;

    let is_authorized = authorization
        .is_action_allowed(
            AuthzUser {
                user_id: user.user_id.to_owned(),
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
