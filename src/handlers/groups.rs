#[path = "groups_test.rs"]
#[cfg(test)]
mod groups_test;

use actix_web::{delete, get, http, post, put, web, Error, HttpResponse, Responder};
use anyhow::{anyhow, Result};
use sea_orm::entity::*;
use sea_orm::QueryFilter;
use serde::Deserialize;

use crate::authentication::Claims;
use crate::errors;
use crate::models;
use crate::models::group::Entity as group;
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

#[get("/groups")]
async fn list_groups(
    data: web::Data<AppState>,
    _claims: Claims,
    query: web::Query<QueryParams>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;

    let mut sql_query = sea_orm::Condition::all();

    if let Some(first_name) = &query.first_name {
        sql_query = sql_query.add(models::group::Column::FirstName.eq(first_name.to_owned()));
    }

    if let Some(last_name) = &query.last_name {
        sql_query = sql_query.add(models::group::Column::LastName.eq(last_name.to_owned()));
    };

    let groups: Vec<models::group::Model> = group::find()
        .filter(sql_query)
        .all(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok(web::Json(groups))
}

#[get("/groups/{group_id}")]
async fn get_group(
    data: web::Data<AppState>,
    claims: Claims,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let group_id = &path.into_inner();

    let conn = &data.conn;

    let group: models::group::Model = (|| -> Result<_, Error> {
        let group_id_uuid = uuid::Uuid::parse_str(group_id)
            .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;
        Ok(group::find_by_id(group_id_uuid).one(conn))
    })()?
    .await
    .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?
    .ok_or(errors::ServerError::NotFound)?;

    Ok(web::Json(group))
}

#[post("/groups")]
async fn create_group(
    data: web::Data<AppState>,
    _claims: Claims,
    body: web::Json<CreateParams>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;

    let name = Set(body.name.to_owned());

    let group: models::group::Model = models::group::ActiveModel {
        group_id: NotSet,
        name,
        created_at: NotSet,
        updated_at: NotSet,
    }
    .insert(conn)
    .await
    .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok((web::Json(group), http::StatusCode::CREATED))
}

#[put("/groups/{group_id}")]
async fn modify_group(
    data: web::Data<AppState>,
    _claims: Claims,
    path: web::Path<String>,
    body: web::Json<ModifyParams>,
) -> Result<impl Responder, Error> {
    let group_id = uuid::Uuid::parse_str(&path.into_inner())
        .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;

    let conn = &data.conn;

    let mut group: models::group::ActiveModel = group::find_by_id(group_id)
        .one(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?
        .ok_or(errors::ServerError::NotFound)?
        .into();

    if body.name.is_some() {
        group.name = Set(body.name.to_owned());
    }

    let group_updated: models::group::Model = group
        .update(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok(web::Json(group_updated))
}

#[delete("/groups/{group_id}")]
async fn delete_group(
    data: web::Data<AppState>,
    _claims: Claims,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;

    let group_id = uuid::Uuid::parse_str(&path.into_inner())
        .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;

    group::delete_by_id(group_id)
        .exec(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok(HttpResponse::NoContent())
}
