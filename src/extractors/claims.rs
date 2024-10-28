use actix_web::{web, Error, FromRequest};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use anyhow::anyhow;
use std::{future::Future, pin::Pin};

use crate::authentication::Claims;
use crate::errors;
use crate::AppState;

impl FromRequest for Claims {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let app_state = req.app_data::<web::Data<AppState>>().unwrap().clone();
        let extractor = BearerAuth::extract(req);
        Box::pin(async move {
            let credentials = extractor
                .await
                .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;
            let token = credentials.token();
            let claims: Claims = app_state
                .authentication
                .validate_token(token.to_string())
                .await
                .map_err(|err| errors::ServerError::Internal(anyhow!(err)))?;
            Ok(claims)
        })
    }
}
