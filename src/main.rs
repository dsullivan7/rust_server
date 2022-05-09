use actix_web::{middleware, web, App, HttpServer};
use sea_orm::DatabaseConnection;
use std::env;

mod authentication;
mod extractors;
mod handlers;
mod models;
mod plaid;
mod services;
mod test_utils;

pub struct AppState {
    conn: DatabaseConnection,
    plaid_client: Box<dyn plaid::IPlaidClient>,
    authentication: Box<dyn authentication::IAuthentication>,
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

    let auth0_domain = std::env::var("AUTH0_DOMAIN").expect("AUTH0_DOMAIN must be set");
    let auth0_audience = std::env::var("AUTH0_AUDIENCE").expect("AUTH0_AUDIENCE must be set");

    let auth = authentication::Authentication {
        audience: auth0_audience,
        domain: auth0_domain,
    };

    let state = web::Data::new(AppState {
        conn,
        plaid_client: Box::new(plaid_client),
        authentication: Box::new(auth),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(middleware::Logger::default())
            .service(handlers::users::get_user)
            .service(handlers::users::list_users)
            .service(handlers::users::create_user)
            .service(handlers::users::modify_user)
            .service(handlers::users::delete_user)
            .service(handlers::bank_accounts::get_bank_account)
            .service(handlers::bank_accounts::list_bank_accounts)
            .service(handlers::bank_accounts::create_bank_account)
            .service(handlers::bank_accounts::modify_bank_account)
            .service(handlers::bank_accounts::delete_bank_account)
            .service(handlers::bank_transfers::get_bank_transfer)
            .service(handlers::bank_transfers::list_bank_transfers)
            .service(handlers::bank_transfers::create_bank_transfer)
            .service(handlers::bank_transfers::modify_bank_transfer)
            .service(handlers::bank_transfers::delete_bank_transfer)
            .service(handlers::plaid::create_token)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
