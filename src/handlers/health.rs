use axum::Json;

use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
}

pub async fn get_health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_owned(),
    })
}
