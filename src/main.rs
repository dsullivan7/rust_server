use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use sea_orm::DatabaseConnection;
use std::env;

mod authentication;
// mod banking;
mod errors;
mod extractors;
mod handlers;
mod models;
mod plaid;

#[cfg(test)]
mod test_utils;

pub struct AppState {
    conn: DatabaseConnection,
    plaid_client: Box<dyn plaid::IPlaidClient>,
    authentication: Box<dyn authentication::IAuthentication>,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("starting!");

    env_logger::init();

    let db_name = env::var("DB_NAME").expect("DB_NAME must be set");
    let db_port = env::var("DB_PORT").expect("DB_PORT must be set");
    let db_host = env::var("DB_HOST").expect("DB_HOST must be set");
    let db_user = env::var("DB_USER").expect("DB_USER must be set");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");

    println!("db_host");
    println!("{}", db_host);

    let db_url = format!("postgres://{db_user}:{db_password}@{db_host}:{db_port}/{db_name}");

    let conn = sea_orm::Database::connect(&db_url)
        .await
        .unwrap_or_else(|err| panic!("error connecting to the database: {:?}", err));

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

    let port = std::env::var("PORT")
        .unwrap_or("7000".to_owned())
        .parse::<u16>()
        .expect("PORT must be a number");

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
        let cors = Cors::permissive();

        App::new()
            .app_data(state.clone())
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .service(handlers::routes())
            .service(handlers::health::get_health)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
