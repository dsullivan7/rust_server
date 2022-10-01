use actix_web::{get, web, Error, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    status: String,
}

#[get("/")]
async fn get_health() -> Result<impl Responder, Error> {
    Ok(web::Json(Response {
        status: "healthy".to_owned(),
    }))
}
