use axum::{
    extract::{Path, Query, State},
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::storage::Database;
use super::{ApiError, ApiResponse, ApiResult, MetricPoint};

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionsQuery {
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub user_id: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct SessionsResponse {
    pub sessions: Vec<SessionData>,
    pub total_count: u64,
    pub page_info: PageInfo,
}

#[derive(Debug, Serialize)]
pub struct SessionData {
    pub id: Uuid,
    pub user_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration_seconds: Option<u64>,
    pub command_count: u64,
    pub tool_usage: Vec<ToolUsage>,
    pub status: SessionStatus,
}

#[derive(Debug, Serialize)]
pub struct ToolUsage {
    pub tool_name: String,
    pub usage_count: u64,
}

#[derive(Debug, Serialize)]
pub enum SessionStatus {
    Active,
    Completed,
    Terminated,
}

#[derive(Debug, Serialize)]
pub struct PageInfo {
    pub has_next: bool,
    pub has_prev: bool,
    pub current_page: u32,
    pub total_pages: u32,
}

pub fn routes() -> Router<Arc<dyn Database>> {
    Router::new()
        .route("/", get(get_sessions))
        .route("/:id", get(get_session_by_id))
        .route("/:id/metrics", get(get_session_metrics))
}

// GET /api/sessions - List sessions with pagination
async fn get_sessions(
    State(db): State<Arc<dyn Database>>,
    Query(params): Query<SessionsQuery>,
) -> ApiResult<impl IntoResponse> {
    let limit = params.limit.unwrap_or(20).min(100); // Max 100 per page
    let offset = params.offset.unwrap_or(0);

    // Get sessions from database
    let sessions_db = db.list_sessions(
        params.user_id.as_deref(),
        limit,
        offset
    ).await?;

    // Convert to API format
    let sessions: Vec<SessionData> = sessions_db
        .into_iter()
        .map(|s| {
            let duration_seconds = if let Some(end_time) = s.end_time {
                Some((end_time - s.start_time).num_seconds() as u64)
            } else {
                None
            };

            let status = if s.end_time.is_some() {
                SessionStatus::Completed
            } else {
                SessionStatus::Active
            };

            // Mock tool usage (TODO: implement real tool tracking)
            let tool_usage = vec![
                ToolUsage { tool_name: "Read".to_string(), usage_count: 5 },
                ToolUsage { tool_name: "Write".to_string(), usage_count: 3 },
                ToolUsage { tool_name: "Edit".to_string(), usage_count: 2 },
            ];

            SessionData {
                id: s.id,
                user_id: s.user_id,
                start_time: s.start_time,
                end_time: s.end_time,
                duration_seconds,
                command_count: s.command_count,
                tool_usage,
                status,
            }
        })
        .collect();

    // Calculate pagination info
    let total_count = sessions.len() as u64; // TODO: get real total count
    let current_page = (offset / limit) + 1;
    let total_pages = (total_count + limit as u64 - 1) / limit as u64;

    let page_info = PageInfo {
        has_next: offset + limit < total_count as u32,
        has_prev: offset > 0,
        current_page,
        total_pages: total_pages as u32,
    };

    let response = SessionsResponse {
        sessions,
        total_count,
        page_info,
    };

    Ok(Json(ApiResponse::success(response)))
}

// GET /api/sessions/:id - Get session details
async fn get_session_by_id(
    State(db): State<Arc<dyn Database>>,
    Path(id): Path<Uuid>,
) -> ApiResult<impl IntoResponse> {
    // Get session from database
    let session_db = db.get_session(id).await?
        .ok_or(ApiError::NotFound)?;

    let duration_seconds = if let Some(end_time) = session_db.end_time {
        Some((end_time - session_db.start_time).num_seconds() as u64)
    } else {
        None
    };

    let status = if session_db.end_time.is_some() {
        SessionStatus::Completed
    } else {
        SessionStatus::Active
    };

    // Mock detailed tool usage for session
    let tool_usage = vec![
        ToolUsage { tool_name: "Read".to_string(), usage_count: 12 },
        ToolUsage { tool_name: "Write".to_string(), usage_count: 8 },
        ToolUsage { tool_name: "Edit".to_string(), usage_count: 5 },
        ToolUsage { tool_name: "Bash".to_string(), usage_count: 3 },
        ToolUsage { tool_name: "Grep".to_string(), usage_count: 2 },
    ];

    let session_data = SessionData {
        id: session_db.id,
        user_id: session_db.user_id,
        start_time: session_db.start_time,
        end_time: session_db.end_time,
        duration_seconds,
        command_count: session_db.command_count,
        tool_usage,
        status,
    };

    Ok(Json(ApiResponse::success(session_data)))
}

// GET /api/sessions/:id/metrics - Get metrics for a specific session
async fn get_session_metrics(
    State(db): State<Arc<dyn Database>>,
    Path(id): Path<Uuid>,
) -> ApiResult<impl IntoResponse> {
    // Verify session exists
    let _session = db.get_session(id).await?
        .ok_or(ApiError::NotFound)?;

    // Get metrics for this session
    let metrics = db.get_metrics(None, None, None).await?;
    
    // Filter metrics that belong to this session (if session_id is tracked)
    // For now, return empty since we don't have session linking implemented
    let session_metrics: Vec<MetricPoint> = metrics
        .into_iter()
        .filter_map(|m| {
            // TODO: Implement proper session-metric linking
            // For now, return some mock data
            if m.name.contains("session") {
                Some(MetricPoint {
                    timestamp: m.timestamp,
                    name: m.name,
                    value: m.value,
                    labels: m.labels,
                })
            } else {
                None
            }
        })
        .collect();

    Ok(Json(ApiResponse::success(session_metrics)))
}