#[path = "bank_accounts_test.rs"]
#[cfg(test)]
mod bank_accounts_test;

use actix_web::{delete, get, http, post, put, web, Error, HttpResponse, Responder};
use sea_orm::entity::*;
use sea_orm::QueryFilter;
use serde::Deserialize;

use uuid::Uuid;

use crate::models;
use crate::models::bank_account::Entity as BankAccount;
use crate::AppState;

#[derive(Deserialize)]
struct QueryParams {
    user_id: Option<String>,
}

#[derive(Deserialize)]
struct CreateParams {
    user_id: Uuid,
}

#[get("/bank-accounts")]
async fn list_bank_accounts(
    data: web::Data<AppState>,
    query: web::Query<QueryParams>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;

    let mut sql_query = sea_orm::Condition::all();

    if query.user_id.is_some() {
        let user_id = uuid::Uuid::parse_str(&query.user_id.as_ref().unwrap()).unwrap();
        sql_query = sql_query.add(models::bank_account::Column::UserId.eq(user_id));
    }

    let bank_accounts: Vec<models::bank_account::Model> = BankAccount::find()
        .filter(sql_query)
        .all(conn)
        .await
        .unwrap();

    Ok(web::Json(bank_accounts))
}

#[get("/bank-accounts/{bank_account_id}")]
async fn get_bank_account(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let bank_account_id = uuid::Uuid::parse_str(&path.into_inner()).unwrap();

    let conn = &data.conn;

    let bank_account: models::bank_account::Model = BankAccount::find_by_id(bank_account_id)
        .one(conn)
        .await
        .unwrap()
        .unwrap();

    Ok(web::Json(bank_account))
}

#[post("/bank-accounts")]
async fn create_bank_account(
    data: web::Data<AppState>,
    body: web::Json<CreateParams>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;

    let user_id = Set(body.user_id.to_owned());

    let bank_account: models::bank_account::Model = models::bank_account::ActiveModel {
        bank_account_id: NotSet,
        user_id,
        plaid_account_id: NotSet,
        plaid_access_token: NotSet,
        dwolla_funding_source_id: NotSet,
        created_at: NotSet,
        updated_at: NotSet,
    }
    .insert(conn)
    .await
    .unwrap();

    Ok((web::Json(bank_account), http::StatusCode::CREATED))
}

#[put("/bank-accounts/{bank_account_id}")]
async fn modify_bank_account(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let bank_account_id = uuid::Uuid::parse_str(&path.into_inner()).unwrap();

    let conn = &data.conn;

    let bank_account: models::bank_account::ActiveModel = BankAccount::find_by_id(bank_account_id)
        .one(conn)
        .await
        .unwrap()
        .unwrap()
        .into();

    let bank_account_updated: models::bank_account::Model =
        bank_account.update(conn).await.unwrap();

    Ok(web::Json(bank_account_updated))
}

#[delete("/bank-accounts/{bank_account_id}")]
async fn delete_bank_account(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;
    let bank_account_id = uuid::Uuid::parse_str(&path.into_inner()).unwrap();
    BankAccount::delete_by_id(bank_account_id)
        .exec(conn)
        .await
        .unwrap();
    Ok(HttpResponse::NoContent())
}
