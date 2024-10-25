#[path = "group_users_test.rs"]
#[cfg(test)]
mod group_users_test;

use actix_web::{delete, get, http, post, put, web, Error, HttpResponse, Responder};
use anyhow::{anyhow, Result};
use sea_orm::entity::*;
use sea_orm::QueryFilter;
use serde::Deserialize;
use uuid::Uuid;

use crate::authentication::Claims;
use crate::errors;
use crate::models;
use crate::models::group_user::Entity as GroupUser;
use crate::AppState;

#[derive(Deserialize)]
struct QueryParams {
    user_id: Option<Uuid>,
    group_id: Option<Uuid>,
}

#[derive(Deserialize)]
struct CreateParams {
    group_id: Uuid,
    user_id: Uuid,
}

#[derive(Deserialize)]
struct ModifyParams {}

#[get("/group-users")]
async fn list_group_users(
    data: web::Data<AppState>,
    _claims: Claims,
    query: web::Query<QueryParams>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;

    let mut sql_query = sea_orm::Condition::all();

    if let Some(user_id) = &query.user_id {
        sql_query = sql_query.add(models::group_user::Column::UserId.eq(user_id.to_owned()));
    }

    if let Some(group_id) = &query.group_id {
        sql_query = sql_query.add(models::group_user::Column::GroupId.eq(group_id.to_owned()));
    }

    let group_users: Vec<models::group_user::Model> = GroupUser::find()
        .filter(sql_query)
        .all(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok(web::Json(group_users))
}

#[get("/group-users/{group_user_id}")]
async fn get_group_user(
    data: web::Data<AppState>,
    _claims: Claims,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let group_user_id = &path.into_inner();

    let conn = &data.conn;

    let group_user: models::group_user::Model = (|| -> Result<_, Error> {
        let group_user_id_uuid = uuid::Uuid::parse_str(group_user_id)
            .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;
        Ok(GroupUser::find_by_id(group_user_id_uuid).one(conn))
    })()?
    .await
    .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?
    .ok_or(errors::ServerError::NotFound)?;

    Ok(web::Json(group_user))
}

#[post("/group-users")]
async fn create_group_user(
    data: web::Data<AppState>,
    _claims: Claims,
    body: web::Json<CreateParams>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;

    let group_id = Set(body.group_id);
    let user_id = Set(body.user_id);

    let group_user: models::group_user::Model = models::group_user::ActiveModel {
        group_user_id: NotSet,
        group_id,
        user_id,
        created_at: NotSet,
        updated_at: NotSet,
    }
    .insert(conn)
    .await
    .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok((web::Json(group_user), http::StatusCode::CREATED))
}

#[put("/group-users/{group_user_id}")]
async fn modify_group_user(
    data: web::Data<AppState>,
    _claims: Claims,
    path: web::Path<String>,
    _body: web::Json<ModifyParams>,
) -> Result<impl Responder, Error> {
    let group_user_id = uuid::Uuid::parse_str(&path.into_inner())
        .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;

    let conn = &data.conn;

    let group_user_found: models::group_user::ActiveModel = GroupUser::find_by_id(group_user_id)
        .one(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?
        .ok_or(errors::ServerError::NotFound)?
        .into();

    let group_user_updated: models::group_user::Model = group_user_found
        .update(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok(web::Json(group_user_updated))
}

#[delete("/group-users/{group_user_id}")]
async fn delete_group_user(
    data: web::Data<AppState>,
    _claims: Claims,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;

    let group_user_id = uuid::Uuid::parse_str(&path.into_inner())
        .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;

    GroupUser::delete_by_id(group_user_id)
        .exec(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok(HttpResponse::NoContent())
}
