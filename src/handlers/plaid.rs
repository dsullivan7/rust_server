#[path = "plaid_test.rs"]
#[cfg(test)]
mod plaid_test;

use actix_web::{post, web, Error, Responder};
use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::errors;
use crate::AppState;

#[derive(Deserialize)]
struct CreateParams {
    user_id: Option<String>,
}

#[derive(Serialize)]
struct Response {
    value: String,
}

#[post("/plaid/token")]
async fn create_token(
    data: web::Data<AppState>,
    body: web::Json<CreateParams>,
) -> Result<impl Responder, Error> {
    let plaid_client = &data.plaid_client;

    let user_id = body
        .user_id
        .as_ref()
        .ok_or(errors::ServerError::RequiredBodyParameter)?
        .to_owned();

    let token = plaid_client
        .create_token(user_id)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok(web::Json(Response { value: token }))
}
