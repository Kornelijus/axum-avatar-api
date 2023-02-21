use std::{env::var_os, error::Error, net::SocketAddr};

use axum::{
    body::Body,
    extract::multipart::Multipart,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{post, Router},
};

mod errors;

use errors::UploadError;

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

    let headers = &field.headers().clone();
    let bytes = field.bytes().await.unwrap();

    // TODO: compress image before making response
    dbg!(name, content_type, headers);

    let mut res = Response::builder();

    res.headers_mut()
        .expect("valid builder")
        .clone_from(headers);

    Ok(res.status(StatusCode::OK).body(Body::from(bytes)).unwrap())
}

async fn api_image_strip_exif(mut _multipart: Multipart) {
    todo!()
}

async fn api_image_watermark(mut _multipart: Multipart) {
    todo!()
}
