use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("not found")]
    NotFound,
    #[error("invalid uuid error")]
    InvalidUUID(anyhow::Error),
    #[error("required body parameter")]
    RequiredBodyParameter,
    #[error("bad request")]
    BadReqest,
    #[error("internal error")]
    Internal(anyhow::Error),
}

impl ServerError {
    pub fn code(&self) -> String {
        match self {
            Self::NotFound => "not_found".to_owned(),
            Self::BadReqest => "bad_request".to_owned(),
            Self::Internal(_) => "internal".to_owned(),
            Self::InvalidUUID(_) => "invalid_uuid".to_owned(),
            Self::RequiredBodyParameter => "required_body_param".to_owned(),
        }
    }
    pub fn message(&self) -> String {
        match self {
            Self::NotFound => "record not found".to_owned(),
            Self::BadReqest => "bad request".to_owned(),
            Self::Internal(_) => "internal".to_owned(),
            Self::InvalidUUID(_) => "invalid uuid".to_owned(),
            Self::RequiredBodyParameter => "required body parameter".to_owned(),
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    code: String,
    message: String,
}

impl ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        HttpResponse::build(status_code).json(ErrorResponse {
            code: self.code(),
            message: self.message(),
        })
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidUUID(_) => StatusCode::BAD_REQUEST,
            Self::BadReqest => StatusCode::BAD_REQUEST,
            Self::RequiredBodyParameter => StatusCode::BAD_REQUEST,
        }
    }
}
