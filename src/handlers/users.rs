use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sea_orm::entity::*;

use crate::models;
use crate::AppState;

#[get("/users")]
async fn list_users() -> impl Responder {
    HttpResponse::Ok().body("list users")
}

#[get("/users/{user_id}")]
async fn get_user(path: web::Path<String>) -> impl Responder {
    let user_id = path.into_inner();
    HttpResponse::Ok().body(format!("get user {user_id}"))
}

#[post("/users")]
async fn create_user(data: web::Data<AppState>) -> impl Responder {
    println!("here");

    let conn = &data.conn;
    let user = models::user::ActiveModel {
        user_id: NotSet,
        first_name: Set("first_name".to_owned()),
        last_name: Set("last_name".to_owned()),
    }
    .save(conn)
    .await
    .unwrap();

    println!("Inserted: {:?}\n", user);

    HttpResponse::Ok().body("saved")
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
