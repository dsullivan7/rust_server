#[path = "users_test.rs"]
#[cfg(test)]
mod users_test;

use actix_web::{delete, get, post, put, web, Error, HttpResponse, Responder};
use sea_orm::entity::*;
use sea_orm::QueryFilter;

use serde::{Deserialize};

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
async fn list_users(data: web::Data<AppState>, query: web::Query<QueryParams>) -> Result<impl Responder, Error> {
    let db = &data.db;

    let mut sql_query = sea_orm::Condition::all();

    if query.first_name.is_some() {
        let first_name = query.first_name.as_ref().unwrap().clone();
        sql_query = sql_query.add(models::user::Column::FirstName.eq(first_name));
    }

    if query.last_name.is_some() {
        let last_name = query.last_name.as_ref().unwrap().clone();
        sql_query = sql_query.add(models::user::Column::LastName.eq(last_name));
    }

    let users: Vec<models::user::Model> = User::find().filter(sql_query).all(db).await.unwrap();

    Ok(web::Json(users))
}

#[get("/users/{user_id}")]
async fn get_user(path: web::Path<String>) -> impl Responder {
    let user_id = path.into_inner();
    HttpResponse::Ok().body(format!("get user {user_id}"))
}

#[post("/users")]
async fn create_user(data: web::Data<AppState>) -> Result<impl Responder, Error> {
    println!("create");
    let db = &data.db;

    // let mut first_name = NotSet;
    //
    // if body.first_name.is_some() {
    //     first_name = Set(body.first_name.as_ref().unwrap().to_owned());
    // }
    //
    // let mut last_name = NotSet;
    //
    // if body.last_name.is_some() {
    //     last_name = Set(body.last_name.as_ref().unwrap().to_owned());
    // }

    let user: models::user::Model = models::user::ActiveModel {
        user_id: NotSet,
        first_name: Set("first_name".to_owned()),
        last_name: Set("last_name".to_owned()),
        created_at: NotSet,
        updated_at: NotSet,
    }
    .insert(db)
    .await
    .unwrap();

    Ok(web::Json(user))
}

#[put("/users/{user_id}")]
async fn modify_user(path: web::Path<String>) -> impl Responder {
    let user_id = path.into_inner();
    HttpResponse::Ok().body(format!("modify user {user_id}"))
}

#[delete("/users/{user_id}")]
async fn delete_user(path: web::Path<String>) -> impl Responder {
    let user_id = path.into_inner();
    HttpResponse::Ok().body(format!("delete user {user_id}"))
}
