use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use anyhow::anyhow;
use sea_orm::DatabaseConnection;
use std::env;

mod auth0;
mod authentication;
mod banking;
mod captcha;
mod errors;
mod extractors;
mod gov;
mod handlers;
mod linked_in;
mod models;
mod plaid;
mod services;

#[cfg(test)]
mod test_utils;

pub struct AppState {
    conn: DatabaseConnection,
    services: Box<dyn services::IServices>,
    gov_client: Box<dyn gov::IGovernment>,
    plaid_client: Box<dyn plaid::IPlaidClient>,
    banking_client: Box<dyn banking::BankingClient>,
    authentication: Box<dyn authentication::IAuthentication>,
}

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    env_logger::init();

    log::info!("initializing the web server...");

    let db_name = env::var("DB_NAME").expect("DB_NAME must be set");
    let db_port = env::var("DB_PORT").expect("DB_PORT must be set");
    let db_host = env::var("DB_HOST").expect("DB_HOST must be set");
    let db_user = env::var("DB_USER").expect("DB_USER must be set");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");

    let db_url = format!("postgres://{db_user}:{db_password}@{db_host}:{db_port}/{db_name}");

    log::info!("connecting to database");

    let conn = sea_orm::Database::connect(&db_url).await.map_err(|err| {
        log::error!("error connecting to the database: {:?}", err);
        anyhow!(err)
    })?;

    log::info!("connected to database");

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

    let dwolla_api_key = std::env::var("DWOLLA_API_KEY").expect("DWOLLA_API_KEY must be set");
    let dwolla_api_secret =
        std::env::var("DWOLLA_API_SECRET").expect("DWOLLA_API_SECRET must be set");
    let dwolla_api_url = std::env::var("DWOLLA_API_URL").expect("DWOLLA_API_URL must be set");

    let dwolla_client =
        banking::DwollaClient::new(dwolla_api_key, dwolla_api_secret, dwolla_api_url);

    let two_captcha_key = std::env::var("TWO_CAPTCHA_KEY").expect("TWO_CAPTCHA_KEY must be set");

    let cptcha = captcha::TwoCaptcha::new(two_captcha_key);
    let gov_client = gov::Government::new(Box::new(cptcha));

    let services = services::Services {};

    let auth0_domain = std::env::var("AUTH0_DOMAIN").expect("AUTH0_DOMAIN must be set");
    let auth0_audience = std::env::var("AUTH0_AUDIENCE").expect("AUTH0_AUDIENCE must be set");

    let auth = authentication::Authentication {
        audience: auth0_audience,
        domain: auth0_domain,
    };

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "7000".to_owned())
        .parse::<u16>()
        .expect("PORT must be a number");

    let state = web::Data::new(AppState {
        conn,
        services: Box::new(services),
        banking_client: Box::new(dwolla_client),
        gov_client: Box::new(gov_client),
        plaid_client: Box::new(plaid_client),
        authentication: Box::new(auth),
    });

    log::info!("starting the web server...");

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
    .map_err(|err| anyhow!(err))
}
