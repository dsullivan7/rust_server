use std::sync::Arc;

use axum::{middleware, routing::get, Router};
use sea_orm::DatabaseConnection;

use crate::authentication;

use super::authentication as authentication_middleware;

use super::health;

use super::users;

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub authentication: Arc<dyn authentication::IAuthentication>,
}

pub fn router(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(health::get_health))
        .route(
            "/protected",
            get(health::get_health).layer(middleware::from_fn_with_state(
                app_state.clone(),
                authentication_middleware::middleware,
            )),
        )
        .route("/users", get(users::list_users))
        .with_state(app_state)
}
