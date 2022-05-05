#[path = "plaid_test.rs"]
#[cfg(test)]
mod plaid_test;

use actix_web::{post, web, Error, Responder};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Deserialize)]
struct CreateParams {
    user_id: Option<String>,
}

#[derive(Serialize)]
struct TokenResponse {
    value: String,
}

#[post("/plaid/token")]
async fn create_token(
    data: web::Data<AppState>,
    body: web::Json<CreateParams>,
) -> Result<impl Responder, Error> {
    let plaid_client = &data.plaid_client;

    let user_id = body.user_id.as_ref().unwrap().to_owned();

    let token = plaid_client.create_token(user_id).await;

    Ok(web::Json(TokenResponse {
        value: token.to_string(),
    }))
}
