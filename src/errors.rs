use actix_web::{error, error::ResponseError, http::StatusCode, HttpResponse};
use derive_more::{Display, Error};
use serde::Serialize;

#[derive(Debug, Display, Error)]
pub enum ServerError {
    NonExistent,
    Unknown,
}

impl ServerError {
    pub fn code(&self) -> String {
        match self {
            Self::NonExistent => "non_existent".to_string(),
            Self::Unknown => "internal".to_string(),
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    code: String,
    message: String,
}

impl error::ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        HttpResponse::build(status_code).json(ErrorResponse {
            code: self.code(),
            message: self.to_string(),
        })
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Self::NonExistent => StatusCode::FORBIDDEN,
            Self::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
