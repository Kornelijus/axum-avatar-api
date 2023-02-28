use std::{env::var_os, error::Error, io::Cursor, net::SocketAddr};

use axum::{
    body::Body,
    extract::multipart::Multipart,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{post, Router},
};

use image::{io::Reader as ImageReader, DynamicImage, ImageFormat};

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

async fn api_image_compress(multipart: Multipart) -> Result<impl IntoResponse, UploadError> {
    let image_field = multipart_image_field(multipart, "image").await?;

    let MultipartImageField {
        name,
        content_type,
        headers,
        ..
    } = &image_field;

    // TODO: compress image here
    dbg!(name, content_type, headers);

    Ok(multipart_image_response(&image_field).await?)
}

async fn api_image_strip_exif(mut _multipart: Multipart) {
    todo!()
}

async fn api_image_watermark(mut _multipart: Multipart) {
    todo!()
}

#[derive(Debug)]
struct MultipartImageField {
    name: String,
    content_type: String,
    filename: Option<String>,
    headers: HeaderMap,
    image: DynamicImage,
    format: ImageFormat,
}

async fn multipart_image_field(
    mut multipart: Multipart,
    field_name: impl Into<String>,
) -> Result<MultipartImageField, UploadError> {
    let field_name: String = field_name.into();

    let Some(field) = multipart.next_field().await? else {
        return Err(UploadError::MissingField { name: field_name });
    };

    let Some(name) = field.name().map(String::from) else {
        return Err(UploadError::MissingFieldName);
    };

    let Some(content_type) = field.content_type().map(String::from) else {
        return Err(UploadError::MissingContentType { name });
    };

    if name != field_name {
        return Err(UploadError::InvalidFieldName { name });
    }

    let Some(format) = ImageFormat::from_mime_type(&content_type) else {
        return Err(UploadError::InvalidContentType { name, content_type });
    };

    let headers = &field.headers().clone();
    let filename = field.file_name().map(String::from);
    let bytes = field.bytes().await?;

    let image = ImageReader::with_format(Cursor::new(&bytes), format).decode()?;

    Ok(MultipartImageField {
        name,
        content_type,
        headers: headers.clone(),
        image,
        format,
        filename,
    })
}

async fn multipart_image_response(
    image_field: &MultipartImageField,
) -> Result<impl IntoResponse, UploadError> {
    let MultipartImageField {
        content_type,
        image,
        format,
        filename,
        ..
    } = image_field;

    let mut bytes = Vec::new();
    image.write_to(&mut Cursor::new(&mut bytes), *format)?;

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", content_type)
        .header(
            "Content-Disposition",
            format!("attachment; filename=\"{}\"", filename.clone().unwrap()),
        )
        .body(Body::from(bytes))
        .unwrap();

    Ok(response)
}
