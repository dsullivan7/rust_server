use actix_web::{get, web, Error, Responder};
use serde::Serialize;

use crate::authentication::Claims;
use crate::AppState;

#[derive(Serialize)]
struct Response {
    principal: i32,
    interest: i32,
    total: i32,
}

#[get("/users/{user_id}/balances")]
async fn get_balances(
    _data: web::Data<AppState>,
    _claims: Claims,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let _user_id = uuid::Uuid::parse_str(&path.into_inner()).unwrap();

    Ok(web::Json(Response {
        principal: 1000,
        interest: 100,
        total: 1100,
    }))
}
