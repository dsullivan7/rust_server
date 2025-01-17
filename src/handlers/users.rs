use std::sync::Arc;

use axum::extract::{Path, State};
use axum::{Extension, Json};

use sea_orm::entity::*;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use serde::Serialize;
use uuid::Uuid;

use crate::authentication::Claims;
use crate::errors::{self, ServerError};
use crate::models;
use crate::models::user::Entity as User;
use anyhow::anyhow;

use super::AppState;

#[derive(Serialize)]
pub struct UserRespose {
    user_id: Uuid,
    first_name: Option<String>,
    last_name: Option<String>,
    created_at: chrono::DateTime<chrono::FixedOffset>,
    updated_at: chrono::DateTime<chrono::FixedOffset>,
}

pub async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<UserRespose>>, ServerError> {
    let conn = &state.conn;

    let users: Vec<UserRespose> = User::find()
        .all(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?
        .iter()
        .map(|user| UserRespose {
            user_id: user.user_id.to_owned(),
            first_name: user.first_name.to_owned(),
            last_name: user.last_name.to_owned(),
            created_at: user.created_at.to_owned(),
            updated_at: user.updated_at.to_owned(),
        })
        .collect();

    Ok(Json(users))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<UserRespose>, ServerError> {
    let conn = &state.conn;

    let user: UserRespose = (|| -> Result<_, ServerError> {
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
    .map(|user| UserRespose {
        user_id: user.user_id.to_owned(),
        first_name: user.first_name.to_owned(),
        last_name: user.last_name.to_owned(),
        created_at: user.created_at.to_owned(),
        updated_at: user.updated_at.to_owned(),
    })
    .ok_or(errors::ServerError::NotFound)?;

    Ok(Json(user))
}
