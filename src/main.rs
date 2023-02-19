use std::env::var_os;
use std::error::Error;
use std::net::SocketAddr;

use axum::extract::multipart::Multipart;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{post, Router};

async fn api_image_compress(
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap();

        match name {
            "image" => {
                let Some(content_type) = field.content_type() else {
                    return Err((StatusCode::BAD_REQUEST, format!("Missing content type for field '{name}'")));
                };

                if !content_type.starts_with("image/") {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        format!("Invalid content type '{content_type}' for field '{name}'"),
                    ));
                }

                dbg!(name, content_type, field.headers());
            }
            "" => {
                return Err((StatusCode::BAD_REQUEST, "Missing field name".into()));
            }
            _ => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    format!("Invalid field name '{name}'"),
                ));
            }
        }
    }

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
