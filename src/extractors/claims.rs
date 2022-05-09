use actix_web::{error::ResponseError, http::StatusCode, web, Error, FromRequest, HttpResponse};
use actix_web_httpauth::{
    extractors::bearer::BearerAuth, headers::www_authenticate::bearer::Bearer,
};
use derive_more::Display;
use serde::Serialize;
use serde_json::json;
use std::{future::Future, pin::Pin};

use crate::authentication::{AuthError, Claims};
use crate::AppState;

#[derive(Serialize)]
pub struct ErrorMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    pub message: String,
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::Unauthorized().json(json!({
            "message": "Authorization error".to_string(),
        }))
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::UNAUTHORIZED
    }
}

#[derive(Debug, Display)]
pub enum ExtractorError {
    #[display(fmt = "authentication")]
    Authentication(actix_web_httpauth::extractors::AuthenticationError<Bearer>),
}

impl ResponseError for ExtractorError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::Authentication(_) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: None,
                error_description: None,
                message: "Requires authentication".to_string(),
            }),
        }
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::UNAUTHORIZED
    }
}

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
            let credentials = extractor.await.map_err(ExtractorError::Authentication)?;
            let token = credentials.token();
            let claims = app_state
                .authentication
                .validate_token(token.to_string())
                .await?;
            Ok(claims)
        })
    }
}
