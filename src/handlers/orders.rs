// #[path = "orders_test.rs"]
// #[cfg(test)]
// mod orders_test;

use actix_web::{delete, get, http, post, put, web, Error, HttpResponse, Responder};
use anyhow::anyhow;
use chrono;
use sea_orm::entity::*;
use sea_orm::QueryFilter;
use serde::Deserialize;
use uuid::Uuid;

use crate::errors;
use crate::models;
use crate::models::order::Entity as Order;
use crate::AppState;

#[derive(Deserialize)]
struct QueryParams {
    user_id: Option<String>,
}

#[derive(Deserialize)]
struct CreateParams {
    user_id: Uuid,
    amount: i32,
    side: String,
}

#[get("/orders")]
async fn list_orders(
    data: web::Data<AppState>,
    query: web::Query<QueryParams>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;

    let mut sql_query = sea_orm::Condition::all();

    if let Some(user_id) = &query.user_id {
        let user_id = uuid::Uuid::parse_str(user_id)
            .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;
        sql_query = sql_query.add(models::order::Column::UserId.eq(user_id));
    }

    let orders: Vec<models::order::Model> = Order::find()
        .filter(sql_query)
        .all(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok(web::Json(orders))
}

#[get("/orders/{order_id}")]
async fn get_order(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let order_id = uuid::Uuid::parse_str(&path.into_inner())
        .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;

    let conn = &data.conn;

    let order: models::order::Model = Order::find_by_id(order_id)
        .one(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?
        .ok_or(errors::ServerError::NotFound)?;

    Ok(web::Json(order))
}

#[post("/orders")]
async fn create_order(
    data: web::Data<AppState>,
    body: web::Json<CreateParams>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;

    let user_id = Set(body.user_id.to_owned());
    let amount = Set(body.amount.to_owned());
    let side = Set(body.side.to_owned());

    let order: models::order::Model = models::order::ActiveModel {
        parent_order_id: NotSet,
        order_id: NotSet,
        user_id,
        amount,
        side,
        status: Set("complete".to_owned()),
        completed_at: Set(Some(
            chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
        )),
        created_at: NotSet,
        updated_at: NotSet,
    }
    .insert(conn)
    .await
    .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    let user_id = Set(body.user_id.to_owned());
    let amount = Set(body.amount.to_owned());
    let side = Set(body.side.to_owned());

    let child_order: models::order::Model = models::order::ActiveModel {
        parent_order_id: Set(Some(order.order_id)),
        order_id: NotSet,
        user_id,
        amount,
        side,
        status: Set("complete".to_owned()),
        completed_at: Set(Some(
            chrono::Utc::now().with_timezone(&chrono::FixedOffset::east(0)),
        )),
        created_at: NotSet,
        updated_at: NotSet,
    }
    .insert(conn)
    .await
    .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok((web::Json(order), http::StatusCode::CREATED))
}

#[put("/orders/{order_id}")]
async fn modify_order(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let order_id = uuid::Uuid::parse_str(&path.into_inner())
        .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;

    let conn = &data.conn;

    let order: models::order::ActiveModel = Order::find_by_id(order_id)
        .one(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?
        .ok_or(errors::ServerError::NotFound)?
        .into();

    let order_updated: models::order::Model = order
        .update(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok(web::Json(order_updated))
}

#[delete("/orders/{order_id}")]
async fn delete_order(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;
    let order_id = uuid::Uuid::parse_str(&path.into_inner())
        .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;
    Order::delete_by_id(order_id)
        .exec(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;
    Ok(HttpResponse::NoContent())
}
