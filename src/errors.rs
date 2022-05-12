use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("non existent record")]
    NonExistent,
    #[error("unknown")]
    Unknown,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl ServerError {
    pub fn code(&self) -> String {
        match self {
            Self::NonExistent => "non_existent".to_string(),
            Self::Unknown => "unknown".to_string(),
            Self::Internal(_) => "internal".to_string(),
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
            message: self.to_string(),
        })
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::NonExistent => StatusCode::NOT_FOUND,
            Self::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
