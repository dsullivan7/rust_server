#[path = "health_test.rs"]
#[cfg(test)]
mod health_test;

use axum::Json;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HealthResponse {
    status: String,
}

pub async fn get_health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_owned(),
    })
}
