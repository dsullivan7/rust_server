use actix_web::{error, http::StatusCode, App, HttpResponse};
use derive_more::{Display, Error};
use serde_json::json;

#[derive(Debug, Display, Error)]
pub enum ServerError {
    NonExistentError,
}

impl error::ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(json!({
            "code": "non_existent",
        }))
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}
