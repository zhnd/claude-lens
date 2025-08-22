use axum::{
    extract::{Query, State},
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

use crate::storage::Database;
use super::{ApiError, ApiResponse, ApiResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyticsQuery {
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub user_email: Option<String>,
    pub organization_id: Option<String>,
    pub range: Option<String>, // "24h", "7d", "30d"
}

#[derive(Debug, Serialize)]
pub struct ProductivityMetrics {
    pub total_commits: u64,
    pub total_pull_requests: u64,
    pub total_lines_added: u64,
    pub total_lines_removed: u64,
    pub files_changed: u64,
    pub active_repositories: Vec<String>,
    pub productivity_trend: Vec<ProductivityPoint>,
    pub top_contributors: Vec<ContributorStats>,
}

#[derive(Debug, Serialize)]
pub struct ProductivityPoint {
    pub timestamp: DateTime<Utc>,
    pub commits: u64,
    pub pull_requests: u64,
    pub lines_added: u64,
    pub lines_removed: u64,
}

#[derive(Debug, Serialize)]
pub struct ContributorStats {
    pub user_email: String,
    pub commits: u64,
    pub pull_requests: u64,
    pub lines_added: u64,
    pub lines_removed: u64,
}

#[derive(Debug, Serialize)]
pub struct CostAnalytics {
    pub total_cost_usd: f64,
    pub total_input_tokens: u64,
    pub total_output_tokens: u64,
    pub total_cache_creation_tokens: u64,
    pub total_cache_read_tokens: u64,
    pub average_cost_per_session: f64,
    pub cost_trend: Vec<CostPoint>,
    pub model_breakdown: Vec<ModelCostBreakdown>,
    pub top_users_by_cost: Vec<UserCostStats>,
}

#[derive(Debug, Serialize)]
pub struct CostPoint {
    pub timestamp: DateTime<Utc>,
    pub cost_usd: f64,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_creation_tokens: u64,
    pub cache_read_tokens: u64,
}

#[derive(Debug, Serialize)]
pub struct ModelCostBreakdown {
    pub model_name: String,
    pub total_cost_usd: f64,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub sessions: u64,
    pub percentage_of_total: f64,
}

#[derive(Debug, Serialize)]
pub struct UserCostStats {
    pub user_email: String,
    pub total_cost_usd: f64,
    pub total_tokens: u64,
    pub sessions: u64,
    pub avg_cost_per_session: f64,
}

#[derive(Debug, Serialize)]
pub struct EfficiencyMetrics {
    pub tokens_per_commit: f64,
    pub cost_per_commit: f64,
    pub tokens_per_line_of_code: f64,
    pub cost_per_line_of_code: f64,
    pub session_productivity_score: f64,
    pub tool_efficiency: Vec<ToolEfficiencyStats>,
    pub time_to_productivity: Vec<TimeToProductivityPoint>,
}

#[derive(Debug, Serialize)]
pub struct ToolEfficiencyStats {
    pub tool_name: String,
    pub usage_count: u64,
    pub success_rate: f64,
    pub avg_duration_ms: f64,
    pub productivity_correlation: f64,
}

#[derive(Debug, Serialize)]
pub struct TimeToProductivityPoint {
    pub timestamp: DateTime<Utc>,
    pub session_start_to_first_commit_minutes: f64,
    pub session_start_to_first_edit_minutes: f64,
}

#[derive(Debug, Serialize)]
pub struct TrendAnalysis {
    pub range: String,
    pub cost_trend: TrendDirection,
    pub productivity_trend: TrendDirection,
    pub token_efficiency_trend: TrendDirection,
    pub user_adoption_trend: TrendDirection,
    pub forecasted_monthly_cost: f64,
    pub forecasted_monthly_productivity: ProductivityForecast,
}

#[derive(Debug, Serialize)]
pub struct ProductivityForecast {
    pub commits: u64,
    pub pull_requests: u64,
    pub lines_of_code: u64,
}

#[derive(Debug, Serialize)]
pub enum TrendDirection {
    Increasing(f64), // percentage increase
    Decreasing(f64), // percentage decrease
    Stable,
}

pub fn routes() -> Router<Arc<dyn Database>> {
    Router::new()
        .route("/productivity", get(get_productivity_metrics))
        .route("/costs", get(get_cost_analytics))
        .route("/efficiency", get(get_efficiency_metrics))
        .route("/trends", get(get_trend_analysis))
}

// GET /api/analytics/productivity - Productivity metrics and trends
async fn get_productivity_metrics(
    State(db): State<Arc<dyn Database>>,
    Query(params): Query<AnalyticsQuery>,
) -> ApiResult<impl IntoResponse> {
    let (start_time, end_time) = parse_time_range(&params)?;
    
    // TODO: Implement actual database queries for productivity metrics
    // This is a mock implementation showing the expected structure
    
    let productivity = ProductivityMetrics {
        total_commits: 42,
        total_pull_requests: 8,
        total_lines_added: 1247,
        total_lines_removed: 389,
        files_changed: 156,
        active_repositories: vec![
            "claude-scope".to_string(),
            "other-project".to_string(),
        ],
        productivity_trend: generate_mock_productivity_trend(start_time, end_time),
        top_contributors: vec![
            ContributorStats {
                user_email: "developer@example.com".to_string(),
                commits: 25,
                pull_requests: 5,
                lines_added: 800,
                lines_removed: 200,
            },
            ContributorStats {
                user_email: "engineer@example.com".to_string(),
                commits: 17,
                pull_requests: 3,
                lines_added: 447,
                lines_removed: 189,
            },
        ],
    };

    Ok(Json(ApiResponse::success(productivity)))
}

// GET /api/analytics/costs - Cost analysis and token usage
async fn get_cost_analytics(
    State(db): State<Arc<dyn Database>>,
    Query(params): Query<AnalyticsQuery>,
) -> ApiResult<impl IntoResponse> {
    let (start_time, end_time) = parse_time_range(&params)?;
    
    // TODO: Implement actual database queries for cost metrics
    // This is a mock implementation showing the expected structure
    
    let costs = CostAnalytics {
        total_cost_usd: 23.47,
        total_input_tokens: 145_892,
        total_output_tokens: 89_347,
        total_cache_creation_tokens: 12_445,
        total_cache_read_tokens: 78_923,
        average_cost_per_session: 1.84,
        cost_trend: generate_mock_cost_trend(start_time, end_time),
        model_breakdown: vec![
            ModelCostBreakdown {
                model_name: "claude-3-5-sonnet-20241022".to_string(),
                total_cost_usd: 18.32,
                input_tokens: 120_445,
                output_tokens: 67_234,
                sessions: 45,
                percentage_of_total: 78.1,
            },
            ModelCostBreakdown {
                model_name: "claude-3-haiku-20240307".to_string(),
                total_cost_usd: 5.15,
                input_tokens: 25_447,
                output_tokens: 22_113,
                sessions: 12,
                percentage_of_total: 21.9,
            },
        ],
        top_users_by_cost: vec![
            UserCostStats {
                user_email: "developer@example.com".to_string(),
                total_cost_usd: 15.23,
                total_tokens: 189_445,
                sessions: 32,
                avg_cost_per_session: 0.48,
            },
            UserCostStats {
                user_email: "engineer@example.com".to_string(),
                total_cost_usd: 8.24,
                total_tokens: 67_234,
                sessions: 25,
                avg_cost_per_session: 0.33,
            },
        ],
    };

    Ok(Json(ApiResponse::success(costs)))
}

// GET /api/analytics/efficiency - Usage efficiency metrics
async fn get_efficiency_metrics(
    State(db): State<Arc<dyn Database>>,
    Query(params): Query<AnalyticsQuery>,
) -> ApiResult<impl IntoResponse> {
    let (start_time, end_time) = parse_time_range(&params)?;
    
    // TODO: Implement actual efficiency calculations
    // This is a mock implementation showing the expected structure
    
    let efficiency = EfficiencyMetrics {
        tokens_per_commit: 3_472.5,
        cost_per_commit: 0.56,
        tokens_per_line_of_code: 143.2,
        cost_per_line_of_code: 0.019,
        session_productivity_score: 8.2, // out of 10
        tool_efficiency: vec![
            ToolEfficiencyStats {
                tool_name: "Edit".to_string(),
                usage_count: 234,
                success_rate: 97.4,
                avg_duration_ms: 1_250.0,
                productivity_correlation: 0.89,
            },
            ToolEfficiencyStats {
                tool_name: "Read".to_string(),
                usage_count: 456,
                success_rate: 99.1,
                avg_duration_ms: 580.0,
                productivity_correlation: 0.72,
            },
            ToolEfficiencyStats {
                tool_name: "Bash".to_string(),
                usage_count: 123,
                success_rate: 94.3,
                avg_duration_ms: 2_840.0,
                productivity_correlation: 0.65,
            },
        ],
        time_to_productivity: generate_mock_time_to_productivity(start_time, end_time),
    };

    Ok(Json(ApiResponse::success(efficiency)))
}

// GET /api/analytics/trends - Historical trend analysis
async fn get_trend_analysis(
    State(db): State<Arc<dyn Database>>,
    Query(params): Query<AnalyticsQuery>,
) -> ApiResult<impl IntoResponse> {
    let range = params.range.as_deref().unwrap_or("30d");
    
    // TODO: Implement actual trend calculations
    // This is a mock implementation showing the expected structure
    
    let trends = TrendAnalysis {
        range: range.to_string(),
        cost_trend: TrendDirection::Increasing(12.3),
        productivity_trend: TrendDirection::Increasing(8.7),
        token_efficiency_trend: TrendDirection::Decreasing(3.2),
        user_adoption_trend: TrendDirection::Increasing(25.1),
        forecasted_monthly_cost: 67.89,
        forecasted_monthly_productivity: ProductivityForecast {
            commits: 180,
            pull_requests: 35,
            lines_of_code: 8_450,
        },
    };

    Ok(Json(ApiResponse::success(trends)))
}

// Helper functions
fn parse_time_range(params: &AnalyticsQuery) -> ApiResult<(DateTime<Utc>, DateTime<Utc>)> {
    match (&params.start_time, &params.end_time, &params.range) {
        (Some(start), Some(end), _) => Ok((*start, *end)),
        (_, _, Some(range)) => {
            let end_time = Utc::now();
            let start_time = match range.as_str() {
                "1h" => end_time - Duration::hours(1),
                "24h" => end_time - Duration::hours(24),
                "7d" => end_time - Duration::days(7),
                "30d" => end_time - Duration::days(30),
                "90d" => end_time - Duration::days(90),
                _ => return Err(ApiError::InvalidQuery(format!("Invalid range: {}", range))),
            };
            Ok((start_time, end_time))
        }
        _ => {
            // Default to last 24 hours
            let end_time = Utc::now();
            let start_time = end_time - Duration::hours(24);
            Ok((start_time, end_time))
        }
    }
}

// Mock data generators (TODO: Replace with real database queries)
fn generate_mock_productivity_trend(start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<ProductivityPoint> {
    let mut points = Vec::new();
    let duration = end - start;
    let num_points = 24; // 24 data points regardless of range
    
    for i in 0..num_points {
        let timestamp = start + duration * i as i32 / num_points as i32;
        points.push(ProductivityPoint {
            timestamp,
            commits: (i % 3) as u64,
            pull_requests: if i % 8 == 0 { 1 } else { 0 },
            lines_added: (50 + i * 10) as u64,
            lines_removed: (20 + i * 3) as u64,
        });
    }
    
    points
}

fn generate_mock_cost_trend(start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<CostPoint> {
    let mut points = Vec::new();
    let duration = end - start;
    let num_points = 24;
    
    for i in 0..num_points {
        let timestamp = start + duration * i as i32 / num_points as i32;
        points.push(CostPoint {
            timestamp,
            cost_usd: 0.5 + (i as f64 * 0.1),
            input_tokens: (1000 + i * 50) as u64,
            output_tokens: (600 + i * 30) as u64,
            cache_creation_tokens: (100 + i * 5) as u64,
            cache_read_tokens: (200 + i * 10) as u64,
        });
    }
    
    points
}

fn generate_mock_time_to_productivity(start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<TimeToProductivityPoint> {
    let mut points = Vec::new();
    let duration = end - start;
    let num_points = 10;
    
    for i in 0..num_points {
        let timestamp = start + duration * i as i32 / num_points as i32;
        points.push(TimeToProductivityPoint {
            timestamp,
            session_start_to_first_commit_minutes: 15.5 + (i as f64 * 2.3),
            session_start_to_first_edit_minutes: 3.2 + (i as f64 * 0.8),
        });
    }
    
    points
}