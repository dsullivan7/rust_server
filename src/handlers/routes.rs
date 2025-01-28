use std::sync::Arc;

use axum::{middleware, routing::get, routing::post, routing::put, Router};
use sea_orm::DatabaseConnection;

use crate::authentication;
use tower::ServiceBuilder;

use super::authentication as authentication_middleware;

use super::health;

use super::users;

#[derive(Clone)]
pub struct AppState {
    pub conn: Arc<DatabaseConnection>,
    pub authentication: Arc<dyn authentication::IAuthentication>,
}

pub fn router(app_state: AppState) -> Router {
    Router::new().route("/", get(health::get_health)).merge(
        Router::new()
            .route("/users", get(users::list_users))
            .route("/users/{user_id}", get(users::get_user))
            .route("/users", post(users::create_user))
            .route("/users/{user_id}", put(users::modify_user))
            .layer(ServiceBuilder::new().layer(middleware::from_fn_with_state(
                app_state.clone(),
                authentication_middleware::middleware,
            )))
            .with_state(app_state),
    )
}
