use actix_web::{App, HttpServer};

mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(handlers::users::get_user)
            .service(handlers::users::list_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
