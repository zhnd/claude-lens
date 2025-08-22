pub mod metrics;
pub mod sessions;
pub mod analytics;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

use crate::storage::Database;

// Common API response wrapper
#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: Utc::now(),
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message.to_string()),
            timestamp: Utc::now(),
        }
    }
}

// Common data structures
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetricPoint {
    pub timestamp: DateTime<Utc>,
    pub name: String,
    pub value: f64,
    pub labels: HashMap<String, String>,
}

// API Error handling
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] crate::storage::DatabaseError),
    #[error("Invalid query parameter: {0}")]
    InvalidQuery(String),
    #[error("Resource not found")]
    NotFound,
    #[error("Internal server error: {0}")]
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ApiError::Database(ref err) => {
                tracing::error!("Database error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
            }
            ApiError::InvalidQuery(ref msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Resource not found"),
            ApiError::Internal(ref msg) => {
                tracing::error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        };

        let body = Json(ApiResponse::<()>::error(message));
        (status, body).into_response()
    }
}

type ApiResult<T> = Result<T, ApiError>;

// Health check endpoint
async fn health_check() -> impl IntoResponse {
    Json(ApiResponse::success(serde_json::json!({
        "status": "healthy",
        "timestamp": Utc::now(),
        "version": env!("CARGO_PKG_VERSION")
    })))
}

// Create all API routes
pub fn create_routes() -> Router<Arc<dyn Database>> {
    Router::new()
        .route("/health", get(health_check))
        .nest("/metrics", metrics::routes())
        .nest("/sessions", sessions::routes())
        .nest("/analytics", analytics::routes())
}