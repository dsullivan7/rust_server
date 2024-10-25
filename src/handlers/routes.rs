use axum::{routing::get, Router};

use super::health;

pub fn router() -> Router {
    Router::new().route("/", get(health::get_health))
}
