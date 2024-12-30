pub mod client;
pub mod builder;


use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Standard API response structure that provides a consistent format
/// for all API responses
/// 
/// # Fields
/// * `status` - Response status ("success" or "error")
/// * `message` - Human readable message describing the response
/// * `data` - Vector of response payload data
/// * `error` - Optional error message when status is "error"
#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    status: String,
    message: String,
    data: Vec<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}