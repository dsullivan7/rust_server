use actix_web::{post, web, Error, Responder};
use serde::{Deserialize, Serialize};

use crate::authentication::Claims;
use crate::AppState;

#[derive(Serialize)]
struct Response {
    principal: i32,
    balance: i32,
    interest: i32,
}

#[post("/users/{user_id}/balances")]
async fn create_token(
    data: web::Data<AppState>,
    _claims: Claims,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let user_id = uuid::Uuid::parse_str(&path.into_inner()).unwrap();

    Ok(web::Json(Response {
        principal: 1000,
        interest: 100,
        balance: 1100,
    }))
}
