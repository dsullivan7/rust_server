use axum::Json;

use serde::Serialize;

#[derive(Serialize)]
pub struct UserRespose {
    first_name: String,
}

pub async fn list_users() -> Json<Vec<UserRespose>> {
    Json(vec![UserRespose {
        first_name: "first_name".to_owned(),
    }])
}
