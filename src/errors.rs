use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("not found")]
    NotFound,
    // #[error("user error")]
    // User(anyhow::Error, String),
    #[error("invalid uuid error")]
    InvalidUUID(anyhow::Error),
    #[error("internal error")]
    Internal(anyhow::Error),
}

impl ServerError {
    pub fn code(&self) -> String {
        match self {
            Self::NotFound => "not_found".to_owned(),
            // Self::User(_, _) => "user".to_owned(),
            Self::Internal(_) => "internal".to_owned(),
            Self::InvalidUUID(_) => "invalid_uuid".to_owned(),
        }
    }
    pub fn message(&self) -> String {
        match self {
            Self::NotFound => "record not found".to_owned(),
            // Self::User(_, user_message) => user_message.to_owned(),
            Self::Internal(_) => "internal".to_owned(),
            Self::InvalidUUID(_) => "invalid uuid".to_owned(),
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
            // Self::User(_, _) => StatusCode::BAD_REQUEST,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidUUID(_) => StatusCode::BAD_REQUEST,
        }
    }
}
