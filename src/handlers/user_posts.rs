#[path = "user_posts_test.rs"]
#[cfg(test)]
mod user_posts_test;

use actix_web::{delete, get, http, post, put, web, Error, HttpResponse, Responder};
use anyhow::{anyhow, Result};
use sea_orm::entity::*;
use sea_orm::QueryFilter;
use serde::Deserialize;
use uuid::Uuid;

use crate::authentication::Claims;
use crate::errors;
use crate::models;
use crate::models::user_post::Entity as Share;
use crate::AppState;

#[derive(Deserialize)]
struct QueryParams {
    user_id: Option<Uuid>,
    post_id: Option<Uuid>,
}

#[derive(Deserialize)]
struct CreateParams {
    user_id: Uuid,
    post_id: Uuid,
}

#[derive(Deserialize)]
struct ModifyParams {}

#[get("/user-posts")]
async fn list_user_posts(
    data: web::Data<AppState>,
    _claims: Claims,
    query: web::Query<QueryParams>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;

    let mut sql_query = sea_orm::Condition::all();

    if let Some(user_id) = &query.user_id {
        sql_query = sql_query.add(models::user_post::Column::UserId.eq(user_id.to_owned()));
    }

    if let Some(post_id) = &query.post_id {
        sql_query = sql_query.add(models::user_post::Column::PostId.eq(post_id.to_owned()));
    }

    let user_posts: Vec<models::user_post::Model> = Share::find()
        .filter(sql_query)
        .all(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok(web::Json(user_posts))
}

#[get("/user-posts/{user_post_id}")]
async fn get_user_post(
    data: web::Data<AppState>,
    _claims: Claims,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let user_post_id = &path.into_inner();

    let conn = &data.conn;

    let user_post: models::user_post::Model = (|| -> Result<_, Error> {
        let user_post_id_uuid = uuid::Uuid::parse_str(user_post_id)
            .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;
        Ok(Share::find_by_id(user_post_id_uuid).one(conn))
    })()?
    .await
    .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?
    .ok_or(errors::ServerError::NotFound)?;

    Ok(web::Json(user_post))
}

#[post("/user-posts")]
async fn create_user_post(
    data: web::Data<AppState>,
    _claims: Claims,
    body: web::Json<CreateParams>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;

    let user_id = Set(body.user_id);
    let post_id = Set(body.post_id);

    let user_post: models::user_post::Model = models::user_post::ActiveModel {
        user_post_id: NotSet,
        user_id,
        post_id,
        created_at: NotSet,
        updated_at: NotSet,
    }
    .insert(conn)
    .await
    .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok((web::Json(user_post), http::StatusCode::CREATED))
}

#[put("/user-posts/{user_post_id}")]
async fn modify_user_post(
    data: web::Data<AppState>,
    _claims: Claims,
    path: web::Path<String>,
    body: web::Json<ModifyParams>,
) -> Result<impl Responder, Error> {
    let user_post_id = uuid::Uuid::parse_str(&path.into_inner())
        .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;

    let conn = &data.conn;

    let mut user_post_found: models::user_post::ActiveModel = Share::find_by_id(user_post_id)
        .one(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?
        .ok_or(errors::ServerError::NotFound)?
        .into();

    let user_post_updated: models::user_post::Model = user_post_found
        .update(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok(web::Json(user_post_updated))
}

#[delete("/user-posts/{user_post_id}")]
async fn delete_user_post(
    data: web::Data<AppState>,
    _claims: Claims,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;

    let user_post_id = uuid::Uuid::parse_str(&path.into_inner())
        .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;

    Share::delete_by_id(user_post_id)
        .exec(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok(HttpResponse::NoContent())
}
