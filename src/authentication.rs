use anyhow::anyhow;
use async_trait::async_trait;
use axum::{
    body::Body,
    extract::Request,
    http::{self, Response},
    middleware::Next,
};
use derive_more::Display;
use jsonwebtoken::{
    decode, decode_header,
    jwk::{AlgorithmParameters, JwkSet},
    Algorithm, DecodingKey, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Display)]
pub enum AuthError {
    #[display(fmt = "decode")]
    Decode(anyhow::Error),
    #[display(fmt = "not_found")]
    NotFound(String),
    #[display(fmt = "no_token")]
    NoToken(),
    #[display(fmt = "unsupported_algorithm")]
    UnsupportedAlgortithm(AlgorithmParameters),
    #[display(fmt = "auth0_request_failed")]
    RequestFailed(anyhow::Error),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
}

#[async_trait]
pub trait IAuthentication: Send + Sync {
    async fn validate_token(&self, token: String) -> Result<Claims, AuthError>;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Authentication {
    pub audience: String,
    pub domain: String,
}

async fn authentication_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, AuthError> {
    let mut auth_header = req
        .headers_mut()
        .get(http::header::AUTHORIZATION)
        .ok_or_else(|| AuthError::NoToken())?
        .to_str()
        .map_err(|err| AuthError::Decode(anyhow!(err)))?
        .split_whitespace();
    let (bearer, token) = (auth_header.next(), auth_header.next());
    // let auth_header = match auth_header {
    //     Some(header) => header
    //         .to_str()
    //         .map_err(|err| AuthError::Decode(anyhow!(err)))?,
    //     None => return Err(AuthError::NoToken()),
    // };
    // let mut header = auth_header.split_whitespace();
    // let (bearer, token) = (header.next(), header.next());
    // let token_data = match decode_jwt(token.unwrap().to_string()) {
    //     Ok(data) => data,
    //     Err(err) => return Err(AuthError::Decode(err)),
    // };
    Ok(next.run(req).await)
}

#[async_trait]
impl IAuthentication for Authentication {
    async fn validate_token(&self, token: String) -> Result<Claims, AuthError> {
        tracing::debug!("validating token");

        let header =
            decode_header(token.as_str()).map_err(|err| AuthError::Decode(anyhow!(err)))?;

        tracing::debug!("found header");
        let kid = header
            .kid
            .ok_or_else(|| AuthError::NotFound("kid not found in token header".to_string()))?;
        tracing::debug!("found kid");
        let jwks_endpoint = format!("https://{}/.well-known/jwks.json", self.domain);
        tracing::debug!("{}", jwks_endpoint);
        let jwks: JwkSet = reqwest::get(&jwks_endpoint)
            .await
            .map_err(|err| AuthError::RequestFailed(anyhow!(err)))?
            .json()
            .await
            .map_err(|err| AuthError::RequestFailed(anyhow!(err)))?;
        tracing::debug!("found jwks");
        let jwk = jwks
            .find(&kid)
            .ok_or_else(|| AuthError::NotFound("No JWK found for kid".to_string()))?;
        tracing::debug!("matched kid");
        match jwk.clone().algorithm {
            AlgorithmParameters::RSA(ref rsa) => {
                let mut validation = Validation::new(Algorithm::RS256);
                validation.set_audience(&[self.audience.as_str()]);
                validation.set_issuer(&[format!("https://{}/", self.domain)]);
                tracing::debug!("decoding key");
                let key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e)
                    .map_err(|err| AuthError::Decode(anyhow!(err)))?;
                tracing::debug!("decoded key");
                let token = decode::<Claims>(token.as_str(), &key, &validation).map_err(|err| {
                    tracing::error!("decoding error: {:?}", err);
                    AuthError::Decode(anyhow!(err))
                })?;
                tracing::debug!("found claims");
                Ok(token.claims)
            }
            algorithm => Err(AuthError::UnsupportedAlgortithm(algorithm)),
        }
    }
}
