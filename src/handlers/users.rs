use actix_web::{get, web, HttpResponse, Responder};

#[get("/users")]
async fn list_users() -> impl Responder {
    HttpResponse::Ok().body("list users")
}

#[get("/users/{user_id}")]
async fn get_user(path: web::Path<String>) -> impl Responder {
    let user_id = path.into_inner();
    HttpResponse::Ok().body(format!("get user {user_id}"))
}
