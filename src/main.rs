use std::{env::var_os, error::Error, io::Cursor, net::SocketAddr};

use axum::{
    body::Body,
    extract::multipart::Multipart,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{post, Router},
};

use image::{io::Reader as ImageReader, ImageFormat};

mod errors;
use errors::UploadError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route("/compress", post(api_image_compress))
        .route("/strip-exif", post(api_image_strip_exif))
        .route("/watermark", post(api_image_watermark));

    let host_addr: SocketAddr = if let Some(host_var) = var_os("IMAGE_API_HOST") {
        host_var.into_string().expect("host addr parses").parse()?
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
    let Some(field) = multipart.next_field().await? else {
        return Err(UploadError::MissingField { name: "image".to_string() });
    };

    let Some(name) = field.name().map(String::from) else {
        return Err(UploadError::MissingFieldName);
    };

    let Some(content_type) = field.content_type().map(String::from) else {
        return Err(UploadError::MissingContentType { name });
    };

    if name != "image" {
        return Err(UploadError::InvalidFieldName { name });
    }

    let headers = &field.headers().clone();
    let bytes = field.bytes().await?;

    let Some(image_format) = ImageFormat::from_mime_type(&content_type) else {
        return Err(UploadError::InvalidContentType { name, content_type });
    };

    let image = ImageReader::with_format(Cursor::new(&bytes), image_format).decode()?;

    // TODO: compress image here
    dbg!(name, content_type, headers);

    let mut res_bytes: Vec<u8> = Vec::new();
    image.write_to(&mut Cursor::new(&mut res_bytes), image_format)?;

    let mut res = Response::builder();

    res.headers_mut()
        .expect("valid builder")
        .clone_from(headers);

    Ok(res
        .status(StatusCode::OK)
        .body(Body::from(res_bytes))
        .expect("valid request"))
}

async fn api_image_strip_exif(mut _multipart: Multipart) {
    todo!()
}

async fn api_image_watermark(mut _multipart: Multipart) {
    todo!()
}
