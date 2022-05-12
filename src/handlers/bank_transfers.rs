#[path = "bank_transfers_test.rs"]
#[cfg(test)]
mod bank_transfers_test;

use actix_web::{delete, get, http, post, put, web, Error, HttpResponse, Responder};
use anyhow::anyhow;
use sea_orm::entity::*;
use sea_orm::QueryFilter;
use serde::Deserialize;
use uuid::Uuid;

use crate::errors;
use crate::models;
use crate::models::bank_transfer::Entity as BankTransfer;
use crate::AppState;

#[derive(Deserialize)]
struct QueryParams {
    bank_account_id: Option<String>,
}

#[derive(Deserialize)]
struct CreateParams {
    bank_account_id: Uuid,
    amount: i32,
}

#[get("/bank-transfers")]
async fn list_bank_transfers(
    data: web::Data<AppState>,
    query: web::Query<QueryParams>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;

    let mut sql_query = sea_orm::Condition::all();

    if let Some(bank_account_id) = &query.bank_account_id {
        sql_query = sql_query
            .add(models::bank_transfer::Column::BankAccountId.eq(bank_account_id.to_owned()));
    }

    let bank_transfers: Vec<models::bank_transfer::Model> = BankTransfer::find()
        .filter(sql_query)
        .all(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok(web::Json(bank_transfers))
}

#[get("/bank-transfers/{bank_transfer_id}")]
async fn get_bank_transfer(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let bank_transfer_id = uuid::Uuid::parse_str(&path.into_inner())
        .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;

    let conn = &data.conn;

    let bank_transfer: models::bank_transfer::Model = BankTransfer::find_by_id(bank_transfer_id)
        .one(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?
        .ok_or(errors::ServerError::NotFound)?;

    Ok(web::Json(bank_transfer))
}

#[post("/bank-transfers")]
async fn create_bank_transfer(
    data: web::Data<AppState>,
    body: web::Json<CreateParams>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;

    let bank_account_id = Set(body.bank_account_id.to_owned());
    let amount = Set(body.amount.to_owned());

    let bank_transfer: models::bank_transfer::Model = models::bank_transfer::ActiveModel {
        bank_transfer_id: NotSet,
        bank_account_id,
        amount,
        status: Set("pending".to_owned()),
        dwolla_transfer_id: NotSet,
        created_at: NotSet,
        updated_at: NotSet,
    }
    .insert(conn)
    .await
    .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok((web::Json(bank_transfer), http::StatusCode::CREATED))
}

#[put("/bank-transfers/{bank_transfer_id}")]
async fn modify_bank_transfer(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let bank_transfer_id = uuid::Uuid::parse_str(&path.into_inner())
        .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;

    let conn = &data.conn;

    let bank_transfer: models::bank_transfer::ActiveModel =
        BankTransfer::find_by_id(bank_transfer_id)
            .one(conn)
            .await
            .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?
            .ok_or(errors::ServerError::NotFound)?
            .into();

    let bank_transfer_updated: models::bank_transfer::Model = bank_transfer
        .update(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok(web::Json(bank_transfer_updated))
}

#[delete("/bank-transfers/{bank_transfer_id}")]
async fn delete_bank_transfer(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;
    let bank_transfer_id = uuid::Uuid::parse_str(&path.into_inner())
        .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;
    BankTransfer::delete_by_id(bank_transfer_id)
        .exec(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;
    Ok(HttpResponse::NoContent())
}
