use anyhow::anyhow;
use axum::{
    body::Body,
    extract::{Path, Request, State},
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

    let is_authorized = authorization.can_list_users(AuthzUser {
        user_id: user.user_id.to_owned(),
        role: user.role.to_owned(),
    });

    if is_authorized {
        return Ok(next.run(req).await);
    }

    Err(errors::ServerError::Unauthorized)
}

pub async fn can_get_user(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(user_id): Path<String>,
    req: Request,
    next: Next,
) -> Result<Response<Body>, errors::ServerError> {
    let conn = &*state.conn.clone();
    let authorization = state.authorization.clone();

    let user_actor = User::find()
        .filter(models::user::Column::Auth0Id.eq(claims.sub.to_owned()))
        .one(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?
        .ok_or(errors::ServerError::Unauthorized)?;

    let user_resource: models::user::Model = (|| -> Result<_, errors::ServerError> {
        if user_id == "me" {
            return Ok(User::find()
                .filter(models::user::Column::Auth0Id.eq(claims.sub.to_owned()))
                .one(conn));
        }
        let user_id_uuid = uuid::Uuid::parse_str(user_id.as_str())
            .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;
        Ok(User::find_by_id(user_id_uuid).one(conn))
    })()?
    .await
    .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?
    .ok_or(errors::ServerError::NotFound)?;

    let is_authorized = authorization.can_get_user(
        AuthzUser {
            user_id: user_actor.user_id.to_owned(),
            role: user_actor.role.to_owned(),
        },
        user_resource.user_id.to_owned(),
    );

    if is_authorized {
        return Ok(next.run(req).await);
    }

    Err(errors::ServerError::Unauthorized)
}
