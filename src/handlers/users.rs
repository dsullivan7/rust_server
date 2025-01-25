#[path = "users_test.rs"]
#[cfg(test)]
mod users_test;

use axum::extract::{Path, State};
use axum::{Extension, Json};

use sea_orm::entity::*;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::authentication::Claims;
use crate::errors::{self, ServerError};
use crate::models;
use crate::models::user::Entity as User;
use anyhow::anyhow;

use super::AppState;

#[derive(Serialize, Deserialize)]
pub struct UserRespose {
    user_id: Uuid,
    first_name: Option<String>,
    last_name: Option<String>,
    created_at: chrono::DateTime<chrono::FixedOffset>,
    updated_at: chrono::DateTime<chrono::FixedOffset>,
}

#[derive(Serialize, Deserialize)]
pub struct ModifyUser {
    first_name: Option<String>,
    last_name: Option<String>,
}

pub async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<UserRespose>>, ServerError> {
    let conn = &*state.conn.clone();

    let users: Vec<models::user::Model> = User::find()
        .all(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok(Json(
        users
            .iter()
            .map(|user| UserRespose {
                user_id: user.user_id.to_owned(),
                first_name: user.first_name.to_owned(),
                last_name: user.last_name.to_owned(),
                created_at: user.created_at.to_owned(),
                updated_at: user.updated_at.to_owned(),
            })
            .collect(),
    ))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<UserRespose>, ServerError> {
    let conn = &*state.conn.clone();

    let user: models::user::Model = (|| -> Result<_, ServerError> {
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

    Ok(Json(UserRespose {
        user_id: user.user_id.to_owned(),
        first_name: user.first_name.to_owned(),
        last_name: user.last_name.to_owned(),
        created_at: user.created_at.to_owned(),
        updated_at: user.updated_at.to_owned(),
    }))
}

pub async fn modify_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<ModifyUser>,
) -> Result<Json<UserRespose>, ServerError> {
    let conn = &*state.conn.clone();

    let mut user: models::user::ActiveModel = (|| -> Result<_, ServerError> {
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
    .ok_or(errors::ServerError::NotFound)?
    .into();

    if body.first_name.is_some() {
        user.first_name = Set(body.first_name.to_owned());
    }

    if body.last_name.is_some() {
        user.last_name = Set(body.last_name.to_owned());
    }

    let user_updated: models::user::Model = user
        .update(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok(Json(UserRespose {
        user_id: user_updated.user_id.to_owned(),
        first_name: user_updated.first_name.to_owned(),
        last_name: user_updated.last_name.to_owned(),
        created_at: user_updated.created_at.to_owned(),
        updated_at: user_updated.updated_at.to_owned(),
    }))
}
