use anyhow::anyhow;
use async_trait::async_trait;
use derive_more::Display;
use jsonwebtoken::{
    decode, decode_header,
    jwk::{AlgorithmParameters, JwkSet},
    Algorithm, DecodingKey, Validation,
};
use serde::{Deserialize, Serialize};

use mockall::*;

#[derive(Debug, Display)]
pub enum AuthError {
    #[display(fmt = "decode")]
    Decode(anyhow::Error),
    #[display(fmt = "not_found")]
    NotFound(String),
    #[display(fmt = "unsupported_algorithm")]
    UnsupportedAlgortithm(AlgorithmParameters),
    #[display(fmt = "auth0_request_failed")]
    RequestFailed(anyhow::Error),
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
        log::debug!("validating token");

        let header =
            decode_header(token.as_str()).map_err(|err| AuthError::Decode(anyhow!(err)))?;

        log::debug!("found header");
        let kid = header
            .kid
            .ok_or_else(|| AuthError::NotFound("kid not found in token header".to_string()))?;
        log::debug!("found kid");
        let jwks_endpoint = format!("https://{}/.well-known/jwks.json", self.domain);
        log::debug!("{}", jwks_endpoint);
        let jwks: JwkSet = reqwest::get(&jwks_endpoint)
            .await
            .map_err(|err| AuthError::RequestFailed(anyhow!(err)))?
            .json()
            .await
            .map_err(|err| AuthError::RequestFailed(anyhow!(err)))?;
        log::debug!("found jwks");
        let jwk = jwks
            .find(&kid)
            .ok_or_else(|| AuthError::NotFound("No JWK found for kid".to_string()))?;
        log::debug!("matched kid");
        match jwk.clone().algorithm {
            AlgorithmParameters::RSA(ref rsa) => {
                let mut validation = Validation::new(Algorithm::RS256);
                validation.set_audience(&[self.audience.as_str()]);
                validation.set_issuer(&[format!("https://{}/", self.domain)]);
                log::debug!("decoding key");
                let key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e)
                    .map_err(|err| AuthError::Decode(anyhow!(err)))?;
                log::debug!("decoded key");
                let token = decode::<Claims>(token.as_str(), &key, &validation).map_err(|err| {
                    log::error!("decoding error: {:?}", err);
                    AuthError::Decode(anyhow!(err))
                })?;
                log::debug!("found claims");
                Ok(token.claims)
            }
            algorithm => Err(AuthError::UnsupportedAlgortithm(algorithm).into()),
        }
    }
}
