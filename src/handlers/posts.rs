#[path = "posts_test.rs"]
#[cfg(test)]
mod posts_test;

use actix_web::{delete, get, http, post, put, web, Error, HttpResponse, Responder};
use anyhow::{anyhow, Result};
use sea_orm::entity::*;
use sea_orm::QueryFilter;
use serde::Deserialize;
use uuid::Uuid;

use crate::authentication::Claims;
use crate::errors;
use crate::models;
use crate::models::post::Entity as Post;
use crate::AppState;

#[derive(Deserialize)]
struct QueryParams {
    group_id: Option<Uuid>,
}

#[derive(Deserialize)]
struct CreateParams {
    group_id: Uuid,
    name: String,
    message: String,
    url: String,
}

#[derive(Deserialize)]
struct ModifyParams {
    name: Option<String>,
    message: Option<String>,
    url: Option<String>,
}

#[get("/posts")]
async fn list_posts(
    data: web::Data<AppState>,
    _claims: Claims,
    query: web::Query<QueryParams>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;

    let mut sql_query = sea_orm::Condition::all();

    if let Some(group_id) = &query.group_id {
        sql_query = sql_query.add(models::post::Column::GroupId.eq(group_id.to_owned()));
    }

    let posts: Vec<models::post::Model> = Post::find()
        .filter(sql_query)
        .all(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok(web::Json(posts))
}

#[get("/posts/{post_id}")]
async fn get_post(
    data: web::Data<AppState>,
    _claims: Claims,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let post_id = &path.into_inner();

    let conn = &data.conn;

    let post: models::post::Model = (|| -> Result<_, Error> {
        let post_id_uuid = uuid::Uuid::parse_str(post_id)
            .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;
        Ok(Post::find_by_id(post_id_uuid).one(conn))
    })()?
    .await
    .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?
    .ok_or(errors::ServerError::NotFound)?;

    Ok(web::Json(post))
}

#[post("/posts")]
async fn create_post(
    data: web::Data<AppState>,
    _claims: Claims,
    body: web::Json<CreateParams>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;

    let group_id = Set(body.group_id.to_owned());
    let name = Set(body.name.to_owned());
    let message = Set(body.message.to_owned());
    let url = Set(body.url.to_owned());

    let post: models::post::Model = models::post::ActiveModel {
        post_id: NotSet,
        group_id,
        name,
        message,
        url,
        created_at: NotSet,
        updated_at: NotSet,
    }
    .insert(conn)
    .await
    .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok((web::Json(post), http::StatusCode::CREATED))
}

#[put("/posts/{post_id}")]
async fn modify_post(
    data: web::Data<AppState>,
    _claims: Claims,
    path: web::Path<String>,
    body: web::Json<ModifyParams>,
) -> Result<impl Responder, Error> {
    let post_id = uuid::Uuid::parse_str(&path.into_inner())
        .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;

    let conn = &data.conn;

    let mut post_found: models::post::ActiveModel = Post::find_by_id(post_id)
        .one(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?
        .ok_or(errors::ServerError::NotFound)?
        .into();

    if let Some(name) = &body.name {
        post_found.name = Set(name.to_owned());
    }

    if let Some(message) = &body.message {
        post_found.message = Set(message.to_owned());
    }

    if let Some(url) = &body.url {
        post_found.url = Set(url.to_owned());
    }

    let post_updated: models::post::Model = post_found
        .update(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok(web::Json(post_updated))
}

#[delete("/posts/{post_id}")]
async fn delete_post(
    data: web::Data<AppState>,
    _claims: Claims,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;

    let post_id = uuid::Uuid::parse_str(&path.into_inner())
        .map_err(|err| errors::ServerError::InvalidUUID(anyhow!(err)))?;

    Post::delete_by_id(post_id)
        .exec(conn)
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok(HttpResponse::NoContent())
}
