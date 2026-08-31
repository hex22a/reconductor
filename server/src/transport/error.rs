use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::{
    application::error::{FieldErrors, ServerError},
    constants::{UNAUTHORIZED_ERROR_MESSAGE, UNEXPECTED_ERROR_MESSAGE},
};

#[derive(Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum ErrorCode {
    ValidationError,
    UnexpectedError,
    Unauthorized,
}

#[derive(Serialize)]
#[serde(untagged)]
enum ErrorDetail {
    Message(String),
    Object(ValidationError),
}

#[derive(Serialize)]
struct ErrorResponse {
    code: ErrorCode,
    error: ErrorDetail,
}

#[derive(Serialize)]
struct ValidationError {
    field_errors: FieldErrors,
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    code: ErrorCode::UnexpectedError,
                    error: ErrorDetail::Message(UNEXPECTED_ERROR_MESSAGE.to_string()),
                }),
            )
                .into_response(),
            ServerError::ValidationError(field_errors) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ErrorResponse {
                    code: ErrorCode::ValidationError,
                    error: ErrorDetail::Object(ValidationError { field_errors }),
                }),
            )
                .into_response(),
            ServerError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    code: ErrorCode::Unauthorized,
                    error: ErrorDetail::Message(UNAUTHORIZED_ERROR_MESSAGE.to_string()),
                }),
            )
                .into_response(),
            ServerError::Forbidden => (StatusCode::FORBIDDEN).into_response(),
        }
    }
}
