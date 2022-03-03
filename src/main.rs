use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sea_orm::DatabaseConnection;
use std::env;

mod handlers;
mod models;

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_name = env::var("DB_NAME").expect("DB_NAME must be set");
    let db_port = env::var("DB_PORT").expect("DB_PORT must be set");
    let db_host = env::var("DB_HOST").expect("DB_HOST must be set");
    let db_user = env::var("DB_USER").expect("DB_USER must be set");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");

    let db_url = format!("postgres://{db_user}:{db_password}@{db_host}:{db_port}/{db_name}");

    let conn = sea_orm::Database::connect(&db_url).await.unwrap();
    let state = web::Data::new(AppState { conn });

    tracing_subscriber::fmt::init();

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(handlers::users::get_user)
            .service(handlers::users::create_user)
            .service(handlers::users::list_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
