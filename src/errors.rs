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

    #[error("Failed to encode / decode image")]
    FailedToProcessImage(#[from] image::ImageError),

    #[error("Failed to parse multipart request")]
    FailedToParseMultipart(#[from] axum::extract::multipart::MultipartError),
}

impl IntoResponse for UploadError {
    fn into_response(self) -> Response {
        let status = match self {
            UploadError::FailedToProcessImage { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::BAD_REQUEST,
        };

        (
            status,
            Json(json!({
                "message": self.to_string()
            })),
        )
            .into_response()
    }
}
