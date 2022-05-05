use actix_web::{middleware, web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use sea_orm::DatabaseConnection;
use std::env;

mod authentication;
mod handlers;
mod middlewares;
mod models;
mod plaid;
mod services;

struct AppState {
    conn: DatabaseConnection,
    plaid_client: Box<dyn plaid::IPlaidClient>,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let db_name = env::var("DB_NAME").expect("DB_NAME must be set");
    let db_port = env::var("DB_PORT").expect("DB_PORT must be set");
    let db_host = env::var("DB_HOST").expect("DB_HOST must be set");
    let db_user = env::var("DB_USER").expect("DB_USER must be set");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");

    let db_url = format!("postgres://{db_user}:{db_password}@{db_host}:{db_port}/{db_name}");

    let conn = sea_orm::Database::connect(&db_url).await.unwrap();

    let auth = HttpAuthentication::bearer(middlewares::authentication::validator);

    let plaid_client_id = std::env::var("PLAID_CLIENT_ID").expect("PLAID_CLIENT_ID must be set");
    let plaid_secret = std::env::var("PLAID_SECRET").expect("PLAID_SECRET must be set");
    let plaid_api_url = std::env::var("PLAID_API_URL").expect("PLAID_API_URL must be set");
    let plaid_redirect_uri =
        std::env::var("PLAID_REDIRECT_URI").expect("PLAID_REDIRECT_URI must be set");

    let plaid_client = plaid::PlaidClient::new(
        plaid_client_id,
        plaid_secret,
        plaid_api_url,
        plaid_redirect_uri,
    );

    let state = web::Data::new(AppState {
        conn,
        plaid_client: Box::new(plaid_client),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(middleware::Logger::default())
            .wrap(auth.clone())
            .service(handlers::users::get_user)
            .service(handlers::users::list_users)
            .service(handlers::users::create_user)
            .service(handlers::users::modify_user)
            .service(handlers::users::delete_user)
            .service(handlers::plaid::create_token)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
