// #[path = "profiles_test.rs"]
// #[cfg(test)]
// mod profiles_test;

use actix_web::{http, post, web, Error, HttpRequest, Responder};
use anyhow::anyhow;
use serde::Deserialize;

use crate::errors;
use crate::AppState;

#[derive(Deserialize)]
struct CreateParams {
    username: String,
    password: String,
    portal_type: String,
}

#[post("/profiles")]
async fn create_profile(
    req: HttpRequest,
    data: web::Data<AppState>,
    body: web::Json<CreateParams>,
) -> Result<impl Responder, Error> {
    let gov_client = &data.gov_client;

    let ip_address = req
        .peer_addr()
        .ok_or(errors::ServerError::BadReqest)?
        .ip()
        .to_string();

    let profile = gov_client
        .get_profile(
            body.portal_type.to_owned(),
            body.username.to_owned(),
            body.password.to_owned(),
            ip_address,
        )
        .await
        .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;

    Ok((web::Json(profile), http::StatusCode::CREATED))
}
