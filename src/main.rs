#![allow(dead_code)]

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use anyhow::anyhow;
use sea_orm::DatabaseConnection;
use std::env;

mod auth0;
mod authentication;
mod authorization;
mod errors;
mod extractors;
mod handlers;
mod models;

#[cfg(test)]
mod test_utils;

pub struct AppState {
    conn: DatabaseConnection,
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
