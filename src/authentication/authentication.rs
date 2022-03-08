use alcoholic_jwt::{token_kid, validate, Validation, JWKS};
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::errors::errors::ServiceError;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

pub async fn validate_token(token: &str) -> Result<bool, ServiceError> {
    let authority = std::env::var("AUTH0_DOMAIN").expect("AUTH0_DOMAIN must be set");
    let jwks = fetch_jwks(&format!(
        "https://{}/.well-known/jwks.json",
        authority.as_str(),
    ))
    .await
    .unwrap();

    let validations = vec![
        Validation::Issuer(format!("https://{}/", authority.as_str())),
        Validation::SubjectPresent,
    ];
    let kid = match token_kid(&token) {
        Ok(res) => res.expect("failed to decode kid"),
        Err(_) => return Err(ServiceError::JWKSFetchError),
    };
    let jwk = jwks.find(&kid).expect("Specified key not found in set");
    let res = validate(token, jwk, validations);
    Ok(res.is_ok())
}

async fn fetch_jwks(uri: &str) -> Result<JWKS, Box<dyn Error>> {
    let res = reqwest::get(uri).await?.json::<JWKS>().await?;
    return Ok(res);
}
