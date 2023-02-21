use std::{env::var_os, error::Error, net::SocketAddr};

use axum::{
    extract::multipart::Multipart,
    http::StatusCode,
    response::IntoResponse,
    routing::{post, Router},
    Json,
};

use thiserror::Error;

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
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": self.to_string()
            })),
        )
            .into_response()
    }
}

async fn api_image_compress(
    mut multipart: Multipart,
) -> Result<impl IntoResponse, impl IntoResponse> {
    // not iterating over next_field() here because we only expect one field named "image"
    let Some(field) = multipart.next_field().await.unwrap() else {
        return Err(UploadError::MissingField { name: "image".to_string() });
    };

    let Some(name) = field.name().map(String::from) else {
        return Err(UploadError::InvalidFieldName { name: "".into() });
    };

    let Some(content_type) = field.content_type().map(String::from) else {
        return Err(UploadError::MissingContentType { name });
    };

    if name != "image" {
        return Err(UploadError::InvalidFieldName { name });
    }

    if !content_type.starts_with("image/") {
        return Err(UploadError::InvalidContentType { name, content_type });
    }

    dbg!(name, content_type, field.headers());

    Ok(())
}

async fn api_image_strip_exif(mut _multipart: Multipart) {
    todo!()
}

async fn api_image_watermark(mut _multipart: Multipart) {
    todo!()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route("/compress", post(api_image_compress))
        .route("/strip-exif", post(api_image_strip_exif))
        .route("/watermark", post(api_image_watermark));

    let host_addr: SocketAddr = if let Some(host_var) = var_os("IMAGE_API_HOST") {
        host_var.into_string().expect("parses to String").parse()?
    } else {
        "0.0.0.0:8000".parse()?
    };

    let server = axum::Server::bind(&host_addr).serve(app.into_make_service());
    println!("Image API listening on http://{host_addr}");
    server.await?;

    Ok(())
}
