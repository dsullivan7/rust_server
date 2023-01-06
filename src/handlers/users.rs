#[path = "users_test.rs"]
#[cfg(test)]
mod users_test;

use actix_web::{delete, get, http, post, put, web, Error, HttpResponse, Responder};
use anyhow::{anyhow, Result};
use sea_orm::entity::*;
use sea_orm::QueryFilter;
use serde::Deserialize;

use crate::authentication::Claims;
use crate::errors;
use crate::models;
use crate::models::user::Entity as User;
use crate::AppState;

#[derive(Deserialize)]
struct QueryParams {
    first_name: Option<String>,
    last_name: Option<String>,
}

#[derive(Deserialize)]
struct CreateParams {
    auth0_id: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
}

#[get("/users")]
async fn list_users(
    data: web::Data<AppState>,
    _claims: Claims,
    query: web::Query<QueryParams>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;

    let mut sql_query = sea_orm::Condition::all();

    if let Some(first_name) = &query.first_name {
        sql_query = sql_query.add(models::user::Column::FirstName.eq(first_name.to_owned()));
    }

    if let Some(last_name) = &query.last_name {
        sql_query = sql_query.add(models::user::Column::LastName.eq(last_name.to_owned()));
    };

    let users: Vec<models::user::Model> = User::find()
        .filter(sql_query)
        .all(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok(web::Json(users))
}

#[get("/users/{user_id}")]
async fn get_user(
    data: web::Data<AppState>,
    claims: Claims,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let user_id = &path.into_inner();

    let conn = &data.conn;

    let user: models::user::Model = (|| -> Result<_, Error> {
        if user_id == "me" {
            return Ok(User::find()
                .filter(models::user::Column::Auth0Id.eq(claims.sub))
                .one(conn));
        }
        let user_id_uuid = uuid::Uuid::parse_str(user_id)
            .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;
        Ok(User::find_by_id(user_id_uuid).one(conn))
    })()?
    .await
    .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?
    .ok_or(errors::ServerError::NotFound)?;

    Ok(web::Json(user))
}

#[post("/users")]
async fn create_user(
    data: web::Data<AppState>,
    _claims: Claims,
    body: web::Json<CreateParams>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;

    let user_found_res = User::find()
        .filter(models::user::Column::Auth0Id.eq(body.auth0_id.to_owned()))
        .one(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    if let Some(user_found) = user_found_res {
        return Ok((web::Json(user_found), http::StatusCode::OK));
    }

    let mut first_name = NotSet;

    if body.first_name.is_some() {
        first_name = Set(body.first_name.to_owned());
    }

    let mut last_name = NotSet;

    if body.last_name.is_some() {
        last_name = Set(body.last_name.to_owned());
    }

    let mut auth0_id = NotSet;

    if body.auth0_id.is_some() {
        auth0_id = Set(body.auth0_id.to_owned());
    }

    let user: models::user::Model = models::user::ActiveModel {
        user_id: NotSet,
        dwolla_customer_id: NotSet,
        auth0_id,
        first_name,
        last_name,
        created_at: NotSet,
        updated_at: NotSet,
    }
    .insert(conn)
    .await
    .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok((web::Json(user), http::StatusCode::CREATED))
}

#[put("/users/{user_id}")]
async fn modify_user(
    data: web::Data<AppState>,
    _claims: Claims,
    path: web::Path<String>,
    body: web::Json<CreateParams>,
) -> Result<impl Responder, Error> {
    let user_id = uuid::Uuid::parse_str(&path.into_inner())
        .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;

    let conn = &data.conn;

    let mut user: models::user::ActiveModel = User::find_by_id(user_id)
        .one(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?
        .ok_or(errors::ServerError::NotFound)?
        .into();

    if body.first_name.is_some() {
        user.first_name = Set(body.first_name.to_owned());
    }

    if body.last_name.is_some() {
        user.last_name = Set(body.last_name.to_owned());
    }

    let user_updated: models::user::Model = user
        .update(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok(web::Json(user_updated))
}

#[delete("/users/{user_id}")]
async fn delete_user(
    data: web::Data<AppState>,
    _claims: Claims,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;

    let user_id = uuid::Uuid::parse_str(&path.into_inner())
        .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;

    User::delete_by_id(user_id)
        .exec(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok(HttpResponse::NoContent())
}
