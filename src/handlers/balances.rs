use actix_web::{get, web, Error, Responder};

use crate::authentication::Claims;
use crate::AppState;

#[get("/users/{user_id}/balances")]
async fn get_balances(
    data: web::Data<AppState>,
    _claims: Claims,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let services = &data.services;

    let _user_id = uuid::Uuid::parse_str(&path.into_inner()).unwrap();

    let orders = Vec::new();
    let interest_rate = 0.05;

    let balance = services.get_balance(orders, interest_rate);

    Ok(web::Json(balance))
}
