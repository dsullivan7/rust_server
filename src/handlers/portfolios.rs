use actix_web::{delete, get, http, post, put, web, Error, HttpResponse, Responder};
use anyhow::{anyhow, Result};
use sea_orm::entity::*;
use sea_orm::QueryFilter;
use serde::Deserialize;
use uuid::Uuid;

use crate::authentication::Claims;
use crate::errors;
use crate::models;
use crate::models::portfolio::Entity as Post;
use crate::AppState;

#[derive(Deserialize)]
struct QueryParams {
    user_id: Option<Uuid>,
}

#[derive(Deserialize)]
struct CreateParams {
    user_id: Uuid,
}

#[derive(Deserialize)]
struct ModifyParams {}

#[get("/portfolios")]
async fn list_portfolios(
    data: web::Data<AppState>,
    _claims: Claims,
    query: web::Query<QueryParams>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;

    let mut sql_query = sea_orm::Condition::all();

    if let Some(user_id) = &query.user_id {
        sql_query = sql_query.add(models::portfolio::Column::UserId.eq(user_id.to_owned()));
    }

    let portfolios: Vec<models::portfolio::Model> = Post::find()
        .filter(sql_query)
        .all(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok(web::Json(portfolios))
}

#[get("/portfolios/{portfolio_id}")]
async fn get_portfolio(
    data: web::Data<AppState>,
    _claims: Claims,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let portfolio_id = &path.into_inner();

    let conn = &data.conn;

    let portfolio: models::portfolio::Model = (|| -> Result<_, Error> {
        let portfolio_id_uuid = uuid::Uuid::parse_str(portfolio_id)
            .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;
        Ok(Post::find_by_id(portfolio_id_uuid).one(conn))
    })()?
    .await
    .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?
    .ok_or(errors::ServerError::NotFound)?;

    Ok(web::Json(portfolio))
}

#[post("/portfolios")]
async fn create_portfolio(
    data: web::Data<AppState>,
    _claims: Claims,
    body: web::Json<CreateParams>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;

    let user_id = Set(body.user_id.to_owned());

    let portfolio: models::portfolio::Model = models::portfolio::ActiveModel {
        portfolio_id: NotSet,
        user_id,
        created_at: NotSet,
        updated_at: NotSet,
    }
    .insert(conn)
    .await
    .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok((web::Json(portfolio), http::StatusCode::CREATED))
}

#[put("/portfolios/{portfolio_id}")]
async fn modify_portfolio(
    data: web::Data<AppState>,
    _claims: Claims,
    path: web::Path<String>,
    // body: web::Json<ModifyParams>,
) -> Result<impl Responder, Error> {
    let portfolio_id = uuid::Uuid::parse_str(&path.into_inner())
        .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;

    let conn = &data.conn;

    let portfolio_found: models::portfolio::ActiveModel = Post::find_by_id(portfolio_id)
        .one(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?
        .ok_or(errors::ServerError::NotFound)?
        .into();

    let portfolio_updated: models::portfolio::Model = portfolio_found
        .update(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok(web::Json(portfolio_updated))
}

#[delete("/portfolios/{portfolio_id}")]
async fn delete_portfolio(
    data: web::Data<AppState>,
    _claims: Claims,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;

    let portfolio_id = uuid::Uuid::parse_str(&path.into_inner())
        .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;

    Post::delete_by_id(portfolio_id)
        .exec(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok(HttpResponse::NoContent())
}
