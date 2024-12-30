//! Module for building standardized API responses
//! 
//! This module provides the ApiResponse struct and associated methods for creating
//! consistent response objects that can be returned from API endpoints.
//! The responses include status, message, data payload and optional error information.

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

impl ApiResponse {
    /// Creates a new successful response with the given message and data
    ///
    /// # Arguments
    /// * `message` - Success message to include in response
    /// * `data` - Vector of data values to include in response
    ///
    /// # Returns
    /// * `ApiResponse` with success status and provided message/data
    pub fn success(message: impl Into<String>, data: Vec<Value>) -> Self {
        Self {
            status: "success".to_string(),
            message: message.into(),
            data,
            error: None,
        }
    }

    /// Creates a new error response with the given message and error details
    ///
    /// # Arguments
    /// * `message` - Error message to include in response
    /// * `error` - Detailed error information
    ///
    /// # Returns
    /// * `ApiResponse` with error status and provided messages
    pub fn error(message: impl Into<String>, error: impl Into<String>) -> Self {
        Self {
            status: "error".to_string(),
            message: message.into(),
            data: Vec::new(),
            error: Some(error.into()),
        }
    }

    /// Converts the ApiResponse into a JSON Value
    ///
    /// # Returns
    /// * `Value` - JSON representation of the ApiResponse
    pub fn to_json(&self) -> Value {
        json!(self)
    }
}
