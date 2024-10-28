use axum::{extract::FromRef, routing::get, Router};
use sea_orm::DatabaseConnection;

use crate::authentication;

use super::health;

#[derive(Clone)]
pub struct State {
    pub app_state: AppState,
}

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub authentication: authentication::Authentication,
}

impl FromRef<State> for AppState {
    fn from_ref(state: &State) -> AppState {
        state.app_state.clone()
    }
}

pub fn router() -> Router<State> {
    Router::new().route("/", get(health::get_health))
}
