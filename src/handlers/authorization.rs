use anyhow::anyhow;
use axum::{
    body::Body,
    extract::{Path, Request, State},
    http::Response,
    middleware::Next,
    Extension,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::{
    authentication::Claims,
    authorization::User as AuthzUser,
    errors,
    models::{self, user::Entity as User},
};

use super::AppState;

pub async fn fetch_user_by_auth_id(
    conn: &DatabaseConnection,
    auth0_id: &str,
) -> Result<Option<models::user::Model>, errors::ServerError> {
    User::find()
        .filter(models::user::Column::Auth0Id.eq(auth0_id.to_owned()))
        .one(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))
}

pub async fn fetch_user_by_user_id(
    conn: &DatabaseConnection,
    user_id: Uuid,
) -> Result<Option<models::user::Model>, errors::ServerError> {
    User::find_by_id(user_id)
        .one(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))
}

pub async fn can_list_users(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    req: Request,
    next: Next,
) -> Result<Response<Body>, errors::ServerError> {
    let conn = &*state.conn.clone();
    let authorization = state.authorization.clone();

    let user = fetch_user_by_auth_id(conn, claims.sub.as_str())
        .await?
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

    let user_actor = fetch_user_by_auth_id(conn, claims.sub.as_str())
        .await?
        .ok_or(errors::ServerError::Unauthorized)?;

    let user_resource = match user_id.as_str() {
        "me" => user_actor.clone(),
        _ => {
            let user_id_uuid = uuid::Uuid::parse_str(user_id.as_str())
                .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;
            fetch_user_by_user_id(conn, user_id_uuid)
                .await?
                .ok_or(errors::ServerError::NotFound)?
        }
    };

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
