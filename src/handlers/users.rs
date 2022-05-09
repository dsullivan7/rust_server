#[path = "users_test.rs"]
#[cfg(test)]
mod users_test;

use actix_web::{delete, get, http, post, put, web, Error, HttpResponse, Responder};
use sea_orm::entity::*;
use sea_orm::QueryFilter;
use serde::Deserialize;

use crate::authentication::Claims;
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
    first_name: Option<String>,
    last_name: Option<String>,
}

#[get("/users")]
async fn list_users(
    data: web::Data<AppState>,
    claims: Claims,
    query: web::Query<QueryParams>,
) -> Result<impl Responder, Error> {

    let mut sql_query = sea_orm::Condition::all();

    if query.first_name.is_some() {
        let first_name = query.first_name.as_ref().unwrap().clone();
        sql_query = sql_query.add(models::user::Column::FirstName.eq(first_name));
    }

    if query.last_name.is_some() {
        let last_name = query.last_name.as_ref().unwrap().clone();
        sql_query = sql_query.add(models::user::Column::LastName.eq(last_name));
    }

    let users: Vec<models::user::Model> = User::find().filter(sql_query).all(conn).await.unwrap();

    Ok(web::Json(users))
}

#[get("/users/{user_id}")]
async fn get_user(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let user_id = uuid::Uuid::parse_str(&path.into_inner()).unwrap();

    let conn = &data.conn;

    let user: models::user::Model = User::find_by_id(user_id).one(conn).await.unwrap().unwrap();

    Ok(web::Json(user))
}

#[post("/users")]
async fn create_user(
    data: web::Data<AppState>,
    body: web::Json<CreateParams>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;

    let mut first_name = NotSet;

    if body.first_name.is_some() {
        first_name = Set(body.first_name.as_ref().unwrap().to_owned());
    }

    let mut last_name = NotSet;

    if body.last_name.is_some() {
        last_name = Set(body.last_name.as_ref().unwrap().to_owned());
    }

    let user: models::user::Model = models::user::ActiveModel {
        user_id: NotSet,
        first_name,
        last_name,
        created_at: NotSet,
        updated_at: NotSet,
    }
    .insert(conn)
    .await
    .unwrap();

    Ok((web::Json(user), http::StatusCode::CREATED))
}

#[put("/users/{user_id}")]
async fn modify_user(
    data: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<CreateParams>,
) -> Result<impl Responder, Error> {
    let user_id = uuid::Uuid::parse_str(&path.into_inner()).unwrap();

    let conn = &data.conn;

    let mut user: models::user::ActiveModel = User::find_by_id(user_id)
        .one(conn)
        .await
        .unwrap()
        .unwrap()
        .into();

    if body.first_name.is_some() {
        user.first_name = Set(body.first_name.as_ref().unwrap().to_owned());
    }

    if body.last_name.is_some() {
        user.last_name = Set(body.last_name.as_ref().unwrap().to_owned());
    }

    let user_updated: models::user::Model = user.update(conn).await.unwrap();

    Ok(web::Json(user_updated))
}

#[delete("/users/{user_id}")]
async fn delete_user(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder, Error> {
    let conn = &data.conn;
    let user_id = uuid::Uuid::parse_str(&path.into_inner()).unwrap();
    User::delete_by_id(user_id).exec(conn).await.unwrap();
    Ok(HttpResponse::NoContent())
}
