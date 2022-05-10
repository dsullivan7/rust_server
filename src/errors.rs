use actix_web::{error, error::ResponseError, http::StatusCode, HttpResponse};
use derive_more::{Display, Error};
use serde::Serialize;

#[derive(Debug, Display, Error)]
pub enum ServerError {
    NonExistent,
    Unknown,
}

impl ServerError {
    pub fn name(&self) -> String {
        match self {
            Self::NonExistent => "NonExistent".to_string(),
            Self::Unknown => "Unknown".to_string(),
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    error: String,
    message: String,
}

impl error::ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        HttpResponse::build(self.status_code()).json(ErrorResponse {
            code: status_code.as_u16(),
            message: self.to_string(),
            error: self.name(),
        })
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Self::NonExistent => StatusCode::FORBIDDEN,
            Self::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
