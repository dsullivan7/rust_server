use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use serde_json::json;
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
    #[error("unauthenticated_reason")]
    UnauthenticatedReason(anyhow::Error),
    #[error("unauthenticated")]
    Unauthenticated,
    #[error("unauthorized")]
    Unauthorized,
}

impl ServerError {
    pub fn code(&self) -> String {
        match self {
            Self::NotFound => "not_found".to_owned(),
            Self::BadReqest => "bad_request".to_owned(),
            Self::Internal(_) => "internal".to_owned(),
            Self::InvalidUUID(_) => "invalid_uuid".to_owned(),
            Self::RequiredBodyParameter => "required_body_param".to_owned(),
            Self::UnauthenticatedReason(_) => "unauthenticated".to_owned(),
            Self::Unauthenticated => "unauthenticated".to_owned(),
            Self::Unauthorized => "unauthorized".to_owned(),
        }
    }
    pub fn message(&self) -> String {
        match self {
            Self::NotFound => "record not found".to_owned(),
            Self::BadReqest => "bad request".to_owned(),
            Self::Internal(_) => "internal".to_owned(),
            Self::InvalidUUID(_) => "invalid uuid".to_owned(),
            Self::RequiredBodyParameter => "required body parameter".to_owned(),
            Self::UnauthenticatedReason(_) => "unauthenticated".to_owned(),
            Self::Unauthenticated => "unauthenticated".to_owned(),
            Self::Unauthorized => "unauthorized".to_owned(),
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    code: String,
    message: String,
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response<Body> {
        let body = Json(json!({
            "error": self.code(),
            "message": self.message(),
        }));

        let status_code = match self {
            Self::NotFound => StatusCode::BAD_REQUEST,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidUUID(_) => StatusCode::BAD_REQUEST,
            Self::BadReqest => StatusCode::BAD_REQUEST,
            Self::RequiredBodyParameter => StatusCode::BAD_REQUEST,
            Self::Unauthenticated => StatusCode::UNAUTHORIZED,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::UnauthenticatedReason(_) => StatusCode::UNAUTHORIZED,
        };

        (status_code, body).into_response()
    }
}
