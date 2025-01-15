use axum::extract::State;
use axum::Json;

use sea_orm::EntityTrait;
use serde::Serialize;

use crate::errors::{self, ServerError};
use crate::models::user::Entity as User;
use anyhow::anyhow;

use super::AppState;

#[derive(Serialize)]
pub struct UserRespose {
    first_name: Option<String>,
    last_name: Option<String>,
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
            first_name: user.first_name.to_owned(),
            last_name: user.last_name.to_owned(),
        })
        .collect();

    Ok(Json(users))
}
