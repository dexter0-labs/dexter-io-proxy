use crate::drivers::drives::Drives;



use actix_web::{get, web, HttpResponse, Responder};
use tracing::info;
use serde_json::json;

use crate::api::ApiResponse;

#[get("/drives")]
/// GET /drives
/// Returns a list of all available drives
///
/// ## Returns
/// - `HttpResponse` with ApiResponse payload containing drive information
pub async fn list_drives() -> impl Responder {
    info!("\x1b[0;34mRequest @ /drives\x1b[0m");
    
    let drives = Drives::list();
    let response = ApiResponse::success(
        "Retrieved drive list successfully",
        vec![json!(drives)]
    );

    HttpResponse::Ok().json(response)
}

#[get("/drives/{drive_letter}")]
/// GET /drives/{drive_letter} 
/// Returns information about a specific drive
///
/// ## Parameters
/// - `drive_letter`: The letter of the drive to get information for
///
/// ## Returns
/// - `HttpResponse` with ApiResponse payload containing drive information
pub async fn get_drive(drive_letter: web::Path<String>) -> impl Responder {
    info!("\x1b[0;34mRequest @ /drives/{}\x1b[0m", drive_letter);

    let drive = Drives::get(&drive_letter);
    match drive {
        Some(drive_info) => {
            let response = ApiResponse::success(
                &format!("Retrieved drive {} information successfully", drive_letter),
                vec![json!(drive_info)]
            );
            HttpResponse::Ok().json(response)
        },
        None => {
            let response = ApiResponse {
                status: "error".to_string(),
                message: format!("Drive {} not found", drive_letter),
                data: vec![],
                error: Some("Drive not found".to_string())
            };
            HttpResponse::NotFound().json(response)
        }
    }
}
