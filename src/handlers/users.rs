use axum::extract::State;
use axum::Json;

use sea_orm::EntityTrait;
use serde::Serialize;
use uuid::Uuid;

use crate::errors::{self, ServerError};
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
