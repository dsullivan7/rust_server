#[path = "bank_accounts_test.rs"]
#[cfg(test)]
mod bank_accounts_test;

use actix_web::{delete, get, http, post, put, web, Error, HttpResponse, Responder};
use anyhow::anyhow;
use sea_orm::entity::*;
use sea_orm::QueryFilter;
use serde::Deserialize;
use uuid::Uuid;

use crate::banking;
use crate::errors;
use crate::models;
use crate::models::bank_account::Entity as BankAccount;
use crate::models::user::Entity as User;
use crate::AppState;

#[derive(Deserialize)]
struct QueryParams {
    user_id: Option<String>,
}

#[derive(Deserialize)]
struct CreateParams {
    user_id: Uuid,
    plaid_public_token: String,
}

#[get("/bank-accounts")]
async fn list_bank_accounts(
    data: web::Data<AppState>,
    query: web::Query<QueryParams>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;

    let mut sql_query = sea_orm::Condition::all();

    if let Some(user_id) = &query.user_id {
        let user_id = uuid::Uuid::parse_str(user_id)
            .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;
        sql_query = sql_query.add(models::bank_account::Column::UserId.eq(user_id));
    }

    let bank_accounts: Vec<models::bank_account::Model> = BankAccount::find()
        .filter(sql_query)
        .all(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok(web::Json(bank_accounts))
}

#[get("/bank-accounts/{bank_account_id}")]
async fn get_bank_account(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let bank_account_id = uuid::Uuid::parse_str(&path.into_inner())
        .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;

    let conn = &data.conn;

    let bank_account: models::bank_account::Model = BankAccount::find_by_id(bank_account_id)
        .one(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?
        .ok_or(errors::ServerError::NotFound)?;

    Ok(web::Json(bank_account))
}

#[post("/bank-accounts")]
async fn create_bank_account(
    data: web::Data<AppState>,
    body: web::Json<CreateParams>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;
    let plaid_client = &data.plaid_client;
    // let banking_client = &data.banking_client;

    let user_id = Set(Some(body.user_id.to_owned()));
    let plaid_public_token = body.plaid_public_token.to_owned();

    let user: models::user::Model = User::find_by_id(body.user_id)
        .one(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?
        .ok_or(errors::ServerError::NotFound)?;

    let plaid_account = plaid_client
        .get_access_token(plaid_public_token)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    let plaid_account = plaid_client
        .get_account(plaid_account)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    // let accessor = banking_client
    //     .get_plaid_accessor()
    //     .ok_or(errors::ServerError::Internal(anyhow!(
    //         "accessor must be set"
    //     )))?;
    //
    // let processor_token = plaid_client
    //     .create_processor_token(plaid_account, accessor)
    //     .await
    //     .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    // let bank_user = banking::User {
    //     first_name: user.first_name.ok_or(errors::ServerError::User(
    //         anyhow!("first_name must be set"),
    //         "First name must be set".to_owned(),
    //     ))?,
    //     last_name: user.last_name.ok_or(errors::ServerError::User(
    //         anyhow!("last_name must be set"),
    //         "Last name must be set".to_owned(),
    //     ))?,
    //     dwolla_customer_id: None,
    // };
    //
    // let bank_account_external = banking_client
    //     .create_bank_account(bank_user, processor_token)
    //     .await
    //     .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    let bank_account: models::bank_account::Model = models::bank_account::ActiveModel {
        bank_account_id: NotSet,
        user_id,
        name: Set(plaid_account.name),
        plaid_account_id: Set(plaid_account.account_id),
        plaid_access_token: Set(Some(plaid_account.access_token)),
        dwolla_funding_source_id: NotSet,
        created_at: NotSet,
        updated_at: NotSet,
    }
    .insert(conn)
    .await
    .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok((web::Json(bank_account), http::StatusCode::CREATED))
}

#[put("/bank-accounts/{bank_account_id}")]
async fn modify_bank_account(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let bank_account_id = uuid::Uuid::parse_str(&path.into_inner())
        .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;

    let conn = &data.conn;

    let bank_account: models::bank_account::ActiveModel = BankAccount::find_by_id(bank_account_id)
        .one(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?
        .ok_or(errors::ServerError::NotFound)?
        .into();

    let bank_account_updated: models::bank_account::Model = bank_account
        .update(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok(web::Json(bank_account_updated))
}

#[delete("/bank-accounts/{bank_account_id}")]
async fn delete_bank_account(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;
    let bank_account_id = uuid::Uuid::parse_str(&path.into_inner())
        .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;
    BankAccount::delete_by_id(bank_account_id)
        .exec(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;
    Ok(HttpResponse::NoContent())
}
