use actix_cors::{Cors, CorsMiddleware};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use serde_json::{json, Value};
use std::env::var;
use std::error::Error;
use std::io::Error as IoError;
use std::io::ErrorKind;
use std::io::Result;
use tracing::{error, info, warn};

// crate imports
use crate::api::ApiResponse;

// endpoints
use crate::api::endpoints::drives::list_drives::{get_drive, list_drives};

#[get("/")]
/// GET /
/// This endpoint is used to test the API
///     
/// ## Returns
/// - `HttpResponse` with ApiResponse payload
async fn test_root() -> impl Responder {
    info!("\x1b[0;34mRequest @ /test\x1b[0m");

    let response = ApiResponse::success("API is healthy", vec![json!({"status": "ok"})]);

    HttpResponse::Ok().json(response)
}

/// Main function for the API client
///
/// This function is used to start the API client
///
/// ## Example
/// ```rust
/// let api_futur = api_client();
///
/// let _ = api_futur.await;
/// ```
///
/// ## Returns
/// - `Result<()>`
///
/// ## Errors
/// - `Error`
///
pub async fn api_client() -> Result<()> {
    let port: u16 = var("DEXTER_IO_PROXY_PORT")
        .unwrap_or_else(|_| "8075".to_string())
        .parse()
        .map_err(|e| {
            error!("Failed to parse port number DEXTER_IO_PROXY_PORT: {}", e);
            IoError::new(ErrorKind::InvalidInput, e)
        })?;

    HttpServer::new(move || {
        let cors: Cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new().wrap(cors)
        .service(test_root)
        .service(list_drives)
        .service(get_drive)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
