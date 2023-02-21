use thiserror::Error;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use serde_json::json;

#[derive(Error, Debug)]
pub enum UploadError {
    #[error("Invalid field name '{name}'")]
    InvalidFieldName { name: String },

    #[error("Missing content type for field '{name}'")]
    MissingContentType { name: String },

    #[error("Missing field '{name}'")]
    MissingField { name: String },

    #[error("Invalid content type '{content_type}' for field '{name}'")]
    InvalidContentType { name: String, content_type: String },
}

impl IntoResponse for UploadError {
    fn into_response(self) -> Response {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": self.to_string()
            })),
        )
            .into_response()
    }
}
