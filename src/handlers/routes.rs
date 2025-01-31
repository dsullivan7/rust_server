use std::sync::Arc;

use axum::routing::delete;
use axum::routing::get;
use axum::routing::post;
use axum::routing::put;
use axum::{middleware, Router};
use sea_orm::DatabaseConnection;

use crate::authentication;
use crate::authorization;
use tower::ServiceBuilder;

use super::authentication as authentication_middleware;
use super::authorization as authorization_middleware;

use super::health;

use super::users;

#[derive(Clone)]
pub struct AppState {
    pub conn: Arc<DatabaseConnection>,
    pub authentication: Arc<dyn authentication::IAuthentication>,
    pub authorization: Arc<dyn authorization::IAuthorization>,
}

pub fn router(app_state: AppState) -> Router {
    Router::new().route("/", get(health::get_health)).merge(
        Router::new()
            .route(
                "/users",
                get(users::list_users).layer(middleware::from_fn_with_state(
                    app_state.clone(),
                    authorization_middleware::can_list_users,
                )),
            )
            .route("/users/{user_id}", get(users::get_user))
            .route("/users", post(users::create_user))
            .route("/users/{user_id}", put(users::modify_user))
            .route("/users/{user_id}", delete(users::delete_user))
            .layer(ServiceBuilder::new().layer(middleware::from_fn_with_state(
                app_state.clone(),
                authentication_middleware::middleware,
            )))
            .with_state(app_state),
    )
}
