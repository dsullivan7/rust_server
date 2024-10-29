use std::sync::Arc;

use axum::{middleware, routing::get, Router};
use sea_orm::DatabaseConnection;

use crate::authentication;

use super::authentication as authentication_middleware;

use super::health;

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub authentication: Arc<dyn authentication::IAuthentication>,
}

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(health::get_health)).route(
        "/protected/",
        get(health::get_health).layer(middleware::from_fn(authentication_middleware::middleware)),
    )
}
