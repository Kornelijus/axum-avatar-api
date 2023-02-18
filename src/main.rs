use axum::{routing::post, Router};
use std::env::var_os;
use std::error::Error;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route("/strip-exif", post(|| async { "Hello, World!" }))
        .route("/compress", post(|| async { "Hello, World!" }))
        .route("/watermark", post(|| async { "Hello, World!" }));

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
