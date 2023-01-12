use actix_web::{delete, get, http, post, put, web, Error, HttpResponse, Responder};
use anyhow::{anyhow, Result};
use sea_orm::entity::*;
use sea_orm::QueryFilter;
use serde::Deserialize;
use uuid::Uuid;

use crate::authentication::Claims;
use crate::errors;
use crate::models;
use crate::models::portfolio_tag::Entity as PortfolioTag;
use crate::AppState;

#[derive(Deserialize)]
struct QueryParams {
    portfolio_id: Option<Uuid>,
}

#[derive(Deserialize)]
struct CreateParams {
    portfolio_id: Uuid,
    tag_id: Uuid,
}

#[derive(Deserialize)]
struct ModifyParams {}

#[get("/portfolio-tags")]
async fn list_portfolio_tags(
    data: web::Data<AppState>,
    _claims: Claims,
    query: web::Query<QueryParams>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;

    let mut sql_query = sea_orm::Condition::all();

    if let Some(portfolio_id) = &query.portfolio_id {
        sql_query =
            sql_query.add(models::portfolio_tag::Column::PortfolioId.eq(portfolio_id.to_owned()));
    }

    let portfolio_tags: Vec<models::portfolio_tag::Model> = PortfolioTag::find()
        .filter(sql_query)
        .all(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok(web::Json(portfolio_tags))
}

#[get("/portfolio-tags/{portfolio_tag_id}")]
async fn get_portfolio_tag(
    data: web::Data<AppState>,
    _claims: Claims,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let portfolio_tag_id = &path.into_inner();

    let conn = &data.conn;

    let portfolio_tag: models::portfolio_tag::Model = (|| -> Result<_, Error> {
        let portfolio_tag_id_uuid = uuid::Uuid::parse_str(portfolio_tag_id)
            .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;
        Ok(PortfolioTag::find_by_id(portfolio_tag_id_uuid).one(conn))
    })()?
    .await
    .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?
    .ok_or(errors::ServerError::NotFound)?;

    Ok(web::Json(portfolio_tag))
}

#[post("/portfolio-tags")]
async fn create_portfolio_tag(
    data: web::Data<AppState>,
    _claims: Claims,
    body: web::Json<CreateParams>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;

    let portfolio_id = Set(body.portfolio_id);
    let tag_id = Set(body.tag_id);

    let portfolio_tag: models::portfolio_tag::Model = models::portfolio_tag::ActiveModel {
        portfolio_tag_id: NotSet,
        portfolio_id,
        tag_id,
        created_at: NotSet,
        updated_at: NotSet,
    }
    .insert(conn)
    .await
    .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok((web::Json(portfolio_tag), http::StatusCode::CREATED))
}

#[put("/portfolio-tags/{portfolio_tag_id}")]
async fn modify_portfolio_tag(
    data: web::Data<AppState>,
    _claims: Claims,
    path: web::Path<String>,
    _body: web::Json<ModifyParams>,
) -> Result<impl Responder, Error> {
    let portfolio_tag_id = uuid::Uuid::parse_str(&path.into_inner())
        .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;

    let conn = &data.conn;

    let portfolio_tag_found: models::portfolio_tag::ActiveModel =
        PortfolioTag::find_by_id(portfolio_tag_id)
            .one(conn)
            .await
            .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?
            .ok_or(errors::ServerError::NotFound)?
            .into();

    let portfolio_tag_updated: models::portfolio_tag::Model = portfolio_tag_found
        .update(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok(web::Json(portfolio_tag_updated))
}

#[delete("/portfolio-tags/{portfolio_tag_id}")]
async fn delete_portfolio_tag(
    data: web::Data<AppState>,
    _claims: Claims,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;

    let portfolio_tag_id = uuid::Uuid::parse_str(&path.into_inner())
        .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;

    PortfolioTag::delete_by_id(portfolio_tag_id)
        .exec(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok(HttpResponse::NoContent())
}
