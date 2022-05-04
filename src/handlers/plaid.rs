#[path = "plaid_test.rs"]
#[cfg(test)]
mod plaid_test;

use actix_web::{delete, get, post, put, web, http, HttpResponse, Error, Responder};
use sea_orm::entity::*;
use sea_orm::QueryFilter;
use serde::{Deserialize};

use crate::models;
use crate::models::user::Entity as User;
use crate::AppState;

#[derive(Deserialize)]
struct CreateParams {
    user_id: Option<String>,
}

#[post("/plaid/token")]
async fn create_token(data: web::Data<AppState>, body: web::Json<CreateParams>) -> Result<impl Responder, Error> {
    let plaid = &data.plaid;

    let user_id = body.user_id.unwrap();

    let token = plaid.create_token(user_id);

    Ok(web::Json(serde_json::json!({ value: token })))
}
