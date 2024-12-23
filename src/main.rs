use axum::{routing, Router};

mod error;
mod zoom;

use crate::error::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv()?;

    let router = Router::new().route("/zoom-auth", routing::get(zoom::zoom_auth));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await?;
    axum::serve(listener, router).await.unwrap();

    Ok(())
}
