use axum::{
    extract::{Query, State},
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::storage::Database;
use super::{ApiError, ApiResponse, ApiResult, MetricPoint};

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricsQuery {
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub metric_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimelineQuery {
    pub range: Option<String>, // e.g., "24h", "7d", "30d"
    pub metric_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MetricsOverview {
    pub total_sessions: u64,
    pub active_sessions: u64,
    pub total_commands: u64,
    pub avg_session_duration: f64, // in seconds
    pub top_tools: Vec<ToolUsage>,
    pub recent_activity: Vec<MetricPoint>,
}

#[derive(Debug, Serialize)]
pub struct ToolUsage {
    pub name: String,
    pub count: u64,
    pub percentage: f64,
}

#[derive(Debug, Serialize)]
pub struct TimelineData {
    pub range: String,
    pub points: Vec<MetricPoint>,
    pub summary: TimelineSummary,
}

#[derive(Debug, Serialize)]
pub struct TimelineSummary {
    pub total_points: u64,
    pub avg_value: f64,
    pub min_value: f64,
    pub max_value: f64,
}

pub fn routes() -> Router<Arc<dyn Database>> {
    Router::new()
        .route("/overview", get(get_metrics_overview))
        .route("/timeline", get(get_metrics_timeline))
}

// GET /api/metrics/overview - Overview of all metrics and activity
async fn get_metrics_overview(
    State(db): State<Arc<dyn Database>>,
) -> ApiResult<impl IntoResponse> {
    // Get session counts
    let sessions = db.list_sessions(None, 1000, 0).await?;
    let total_sessions = sessions.len() as u64;
    let active_sessions = sessions.iter()
        .filter(|s| s.end_time.is_none())
        .count() as u64;

    // Calculate total commands and average duration
    let total_commands: u64 = sessions.iter().map(|s| s.command_count).sum();
    let completed_sessions: Vec<_> = sessions.iter()
        .filter(|s| s.end_time.is_some())
        .collect();
    
    let avg_session_duration = if completed_sessions.is_empty() {
        0.0
    } else {
        let total_duration: i64 = completed_sessions.iter()
            .map(|s| {
                let duration = s.end_time.unwrap() - s.start_time;
                duration.num_seconds()
            })
            .sum();
        total_duration as f64 / completed_sessions.len() as f64
    };

    // Mock tool usage data (TODO: implement real tool tracking)
    let top_tools = vec![
        ToolUsage { name: "Read".to_string(), count: 45, percentage: 35.0 },
        ToolUsage { name: "Write".to_string(), count: 28, percentage: 22.0 },
        ToolUsage { name: "Bash".to_string(), count: 25, percentage: 19.5 },
        ToolUsage { name: "Edit".to_string(), count: 20, percentage: 15.6 },
        ToolUsage { name: "Grep".to_string(), count: 10, percentage: 7.8 },
    ];

    // Get recent metrics (last 10 points)
    let recent_metrics = db.get_metrics(
        Some(Utc::now() - Duration::hours(24)),
        Some(Utc::now()),
        None
    ).await?;

    let recent_activity: Vec<MetricPoint> = recent_metrics
        .into_iter()
        .take(10)
        .map(|m| MetricPoint {
            timestamp: m.timestamp,
            name: m.name,
            value: m.value,
            labels: m.labels,
        })
        .collect();

    let overview = MetricsOverview {
        total_sessions,
        active_sessions,
        total_commands,
        avg_session_duration,
        top_tools,
        recent_activity,
    };

    Ok(Json(ApiResponse::success(overview)))
}

// GET /api/metrics/timeline - Time series data with range parameter
async fn get_metrics_timeline(
    State(db): State<Arc<dyn Database>>,
    Query(params): Query<TimelineQuery>,
) -> ApiResult<impl IntoResponse> {
    let range = params.range.as_deref().unwrap_or("24h");
    
    // Parse range parameter
    let (start_time, duration_label) = match range {
        "1h" => (Utc::now() - Duration::hours(1), "1 hour"),
        "24h" => (Utc::now() - Duration::hours(24), "24 hours"),
        "7d" => (Utc::now() - Duration::days(7), "7 days"),
        "30d" => (Utc::now() - Duration::days(30), "30 days"),
        _ => return Err(ApiError::InvalidQuery(format!("Invalid range: {}", range))),
    };

    // Get metrics from database
    let metrics = db.get_metrics(
        Some(start_time),
        Some(Utc::now()),
        params.metric_name.as_deref()
    ).await?;

    // Convert to MetricPoints
    let points: Vec<MetricPoint> = metrics
        .into_iter()
        .map(|m| MetricPoint {
            timestamp: m.timestamp,
            name: m.name,
            value: m.value,
            labels: m.labels,
        })
        .collect();

    // Calculate summary statistics
    let values: Vec<f64> = points.iter().map(|p| p.value).collect();
    let summary = if values.is_empty() {
        TimelineSummary {
            total_points: 0,
            avg_value: 0.0,
            min_value: 0.0,
            max_value: 0.0,
        }
    } else {
        TimelineSummary {
            total_points: values.len() as u64,
            avg_value: values.iter().sum::<f64>() / values.len() as f64,
            min_value: values.iter().fold(f64::INFINITY, |a, &b| a.min(b)),
            max_value: values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)),
        }
    };

    let timeline = TimelineData {
        range: duration_label.to_string(),
        points,
        summary,
    };

    Ok(Json(ApiResponse::success(timeline)))
}

fn parse_duration(range: &str) -> ApiResult<Duration> {
    match range {
        "1h" => Ok(Duration::hours(1)),
        "24h" => Ok(Duration::hours(24)),
        "7d" => Ok(Duration::days(7)),
        "30d" => Ok(Duration::days(30)),
        _ => Err(ApiError::InvalidQuery(format!("Invalid time range: {}", range))),
    }
}