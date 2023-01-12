use actix_web::{delete, get, http, post, put, web, Error, HttpResponse, Responder};
use anyhow::{anyhow, Result};
use sea_orm::entity::*;
use sea_orm::QueryFilter;
use serde::Deserialize;

use crate::authentication::Claims;
use crate::errors;
use crate::models;
use crate::models::tag::Entity as Tag;
use crate::AppState;

#[derive(Deserialize)]
struct QueryParams {
    name: Option<String>,
}

#[derive(Deserialize)]
struct CreateParams {
    name: String,
}

#[derive(Deserialize)]
struct ModifyParams {
    name: Option<String>,
}

#[get("/tags")]
async fn list_tags(
    data: web::Data<AppState>,
    _claims: Claims,
    query: web::Query<QueryParams>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;

    let mut sql_query = sea_orm::Condition::all();

    if let Some(name) = &query.name {
        sql_query = sql_query.add(models::tag::Column::Name.eq(name.to_owned()));
    }

    let tags: Vec<models::tag::Model> = Tag::find()
        .filter(sql_query)
        .all(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok(web::Json(tags))
}

#[get("/tags/{tag_id}")]
async fn get_tag(
    data: web::Data<AppState>,
    _claims: Claims,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let tag_id = &path.into_inner();

    let conn = &data.conn;

    let tag: models::tag::Model = (|| -> Result<_, Error> {
        let tag_id_uuid = uuid::Uuid::parse_str(tag_id)
            .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;
        Ok(Tag::find_by_id(tag_id_uuid).one(conn))
    })()?
    .await
    .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?
    .ok_or(errors::ServerError::NotFound)?;

    Ok(web::Json(tag))
}

#[post("/tags")]
async fn create_tag(
    data: web::Data<AppState>,
    _claims: Claims,
    body: web::Json<CreateParams>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;

    let name = Set(body.name.to_owned());

    let tag: models::tag::Model = models::tag::ActiveModel {
        tag_id: NotSet,
        name,
        created_at: NotSet,
        updated_at: NotSet,
    }
    .insert(conn)
    .await
    .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok((web::Json(tag), http::StatusCode::CREATED))
}

#[put("/tags/{tag_id}")]
async fn modify_tag(
    data: web::Data<AppState>,
    _claims: Claims,
    path: web::Path<String>,
    body: web::Json<ModifyParams>,
) -> Result<impl Responder, Error> {
    let tag_id = uuid::Uuid::parse_str(&path.into_inner())
        .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;

    let conn = &data.conn;

    let mut tag_found: models::tag::ActiveModel = Tag::find_by_id(tag_id)
        .one(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?
        .ok_or(errors::ServerError::NotFound)?
        .into();

    if let Some(name) = &body.name {
        tag_found.name = Set(name.to_owned());
    }

    let tag_updated: models::tag::Model = tag_found
        .update(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok(web::Json(tag_updated))
}

#[delete("/tags/{tag_id}")]
async fn delete_tag(
    data: web::Data<AppState>,
    _claims: Claims,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;

    let tag_id = uuid::Uuid::parse_str(&path.into_inner())
        .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;

    Tag::delete_by_id(tag_id)
        .exec(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok(HttpResponse::NoContent())
}
