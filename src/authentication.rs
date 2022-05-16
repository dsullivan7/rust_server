use jsonwebtoken::{
    decode, decode_header,
    jwk::{AlgorithmParameters, JwkSet},
    Algorithm, DecodingKey, Validation,
};
use serde::{Deserialize, Serialize};

use derive_more::Display;

use async_trait::async_trait;
use mockall::*;

#[derive(Debug, Display)]
pub enum AuthError {
    #[display(fmt = "decode")]
    Decode(jsonwebtoken::errors::Error),
    #[display(fmt = "not_found")]
    NotFound(String),
    #[display(fmt = "unsupported_algorithm")]
    UnsupportedAlgortithm(AlgorithmParameters),
    #[display(fmt = "auth0_request_failed")]
    RequestFailed(reqwest::Error),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
}

#[automock]
#[async_trait]
pub trait IAuthentication: Send + Sync {
    async fn validate_token(&self, token: String) -> Result<Claims, AuthError>;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Authentication {
    pub audience: String,
    pub domain: String,
}

#[async_trait]
impl IAuthentication for Authentication {
    async fn validate_token(&self, token: String) -> Result<Claims, AuthError> {
        log::info!("validating token");

        let header = decode_header(token.as_str()).map_err(AuthError::Decode)?;
        log::info!("found header");
        let kid = header
            .kid
            .ok_or_else(|| AuthError::NotFound("kid not found in token header".to_string()))?;
        log::info!("found kid");
        let jwks: JwkSet = reqwest::get(&format!("https://{}/.well-known/jwks.json", self.domain))
            .await
            .map_err(AuthError::RequestFailed)?
            .json()
            .await
            .map_err(AuthError::RequestFailed)?;
        log::info!("found jwks");
        let jwk = jwks
            .find(&kid)
            .ok_or_else(|| AuthError::NotFound("No JWK found for kid".to_string()))?;
        log::info!("matched kid");
        match jwk.clone().algorithm {
            AlgorithmParameters::RSA(ref rsa) => {
                let mut validation = Validation::new(Algorithm::RS256);
                validation.set_audience(&[self.audience.as_str()]);
                validation.set_issuer(&[format!("https://{}/", self.domain)]);
                let key =
                    DecodingKey::from_rsa_components(&rsa.n, &rsa.e).map_err(AuthError::Decode)?;
                let token = decode::<Claims>(token.as_str(), &key, &validation)
                    .map_err(AuthError::Decode)?;
                log::info!("found claims");
                Ok(token.claims)
            }
            algorithm => Err(AuthError::UnsupportedAlgortithm(algorithm).into()),
        }
    }
}
