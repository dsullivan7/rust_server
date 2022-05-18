use actix_web::{get, web, Error, Responder};

use anyhow::anyhow;
use sea_orm::entity::*;
use sea_orm::QueryFilter;

use crate::authentication::Claims;
use crate::errors;
use crate::models;
use crate::models::order::Entity as Order;
use crate::services;
use crate::AppState;

const INTEREST_RATE: f64 = 0.05;

#[get("/users/{user_id}/balances")]
async fn get_balances(
    data: web::Data<AppState>,
    _claims: Claims,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let services = &data.services;
    let conn = &data.conn;

    let user_id = uuid::Uuid::parse_str(&path.into_inner())
        .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;

    let orders: Vec<services::Order> = Order::find()
        .filter(
            sea_orm::Condition::all()
                .add(models::order::Column::UserId.eq(user_id))
                .add(models::order::Column::ParentOrderId.is_null()),
        )
        .all(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?
        .iter()
        .map(|order| services::Order {
            amount: order.amount,
            side: order.side.to_owned(),
            status: order.status.to_owned(),
        })
        .collect();

    let balance = services.get_balance(orders, INTEREST_RATE);

    Ok(web::Json(balance))
}
