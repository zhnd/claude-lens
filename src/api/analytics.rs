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

// Dashboard-specific data structures
#[derive(Debug, Serialize)]
pub struct DashboardKPIs {
    pub today_sessions: u64,
    pub today_sessions_change: f64, // percentage change from yesterday
    pub total_tokens: u64,
    pub total_tokens_change: f64,
    pub total_cost: f64,
    pub total_cost_change: f64,
    pub lines_of_code: u64,
    pub lines_of_code_change: f64,
    pub period: String, // "today", "24h", "7d", "30d"
}

#[derive(Debug, Serialize)]
pub struct TokenTrendData {
    pub range: String,
    pub data_points: Vec<TokenTrendPoint>,
}

#[derive(Debug, Serialize)]
pub struct TokenTrendPoint {
    pub timestamp: DateTime<Utc>,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_creation_tokens: u64,
    pub cache_read_tokens: u64,
    pub total_tokens: u64,
}

#[derive(Debug, Serialize)]
pub struct ToolUsageData {
    pub total_tool_calls: u64,
    pub tools: Vec<ToolUsageStats>,
}

#[derive(Debug, Serialize)]
pub struct ToolUsageStats {
    pub tool_name: String,
    pub usage_count: u64,
    pub success_rate: f64,
    pub avg_duration_ms: f64,
    pub percentage: f64,
    pub color: String, // for chart coloring
}

#[derive(Debug, Serialize)]
pub struct UsageHeatmapData {
    pub timezone: String,
    pub heatmap: Vec<HeatmapCell>,
}

#[derive(Debug, Serialize)]
pub struct HeatmapCell {
    pub hour: u8,       // 0-23
    pub day_of_week: u8, // 0-6 (Sunday = 0)
    pub intensity: f64,  // 0.0-1.0
    pub session_count: u64,
    pub token_count: u64,
}

// Advanced analytics data structures
#[derive(Debug, Serialize)]
pub struct ModelCostComparison {
    pub models: Vec<ModelCostComparisonItem>,
    pub total_cost: f64,
    pub period: String,
}

#[derive(Debug, Serialize)]
pub struct ModelCostComparisonItem {
    pub model_name: String,
    pub cost_per_session: f64,
    pub total_sessions: u64,
    pub total_cost: f64,
    pub avg_input_tokens: u64,
    pub avg_output_tokens: u64,
    pub efficiency_score: f64, // cost per token
    pub color: String,
}

#[derive(Debug, Serialize)]
pub struct BudgetProgressData {
    pub current_month_cost: f64,
    pub monthly_budget: f64,
    pub percentage_used: f64,
    pub days_remaining: u32,
    pub projected_month_end_cost: f64,
    pub is_over_budget: bool,
    pub daily_breakdown: Vec<DailyCostBreakdown>,
}

#[derive(Debug, Serialize)]
pub struct DailyCostBreakdown {
    pub date: DateTime<Utc>,
    pub cost: f64,
    pub sessions: u64,
    pub tokens: u64,
}

#[derive(Debug, Serialize)]
pub struct AdvancedToolEfficiency {
    pub overall_efficiency_score: f64,
    pub tools: Vec<AdvancedToolStats>,
    pub efficiency_over_time: Vec<EfficiencyTimePoint>,
}

#[derive(Debug, Serialize)]
pub struct AdvancedToolStats {
    pub tool_name: String,
    pub usage_count: u64,
    pub success_rate: f64,
    pub avg_duration_ms: f64,
    pub median_duration_ms: f64,
    pub efficiency_score: f64,
    pub time_saved_estimate_hours: f64,
    pub cost_per_use: f64,
    pub trend: TrendDirection,
}

#[derive(Debug, Serialize)]
pub struct EfficiencyTimePoint {
    pub timestamp: DateTime<Utc>,
    pub overall_score: f64,
    pub top_tool_score: f64,
}

#[derive(Debug, Serialize)]
pub struct SessionDurationDistribution {
    pub total_sessions: u64,
    pub avg_duration_minutes: f64,
    pub median_duration_minutes: f64,
    pub distribution_buckets: Vec<DurationBucket>,
    pub duration_over_time: Vec<DurationTimePoint>,
}

#[derive(Debug, Serialize)]
pub struct DurationBucket {
    pub min_minutes: u32,
    pub max_minutes: u32,
    pub session_count: u64,
    pub percentage: f64,
    pub label: String, // e.g., "0-5 min", "5-15 min"
}

#[derive(Debug, Serialize)]
pub struct DurationTimePoint {
    pub timestamp: DateTime<Utc>,
    pub avg_duration_minutes: f64,
    pub session_count: u64,
}

#[derive(Debug, Serialize)]
pub struct CodeGenerationStats {
    pub total_code_files_generated: u64,
    pub total_lines_generated: u64,
    pub avg_lines_per_file: f64,
    pub most_generated_languages: Vec<LanguageStats>,
    pub generation_over_time: Vec<GenerationTimePoint>,
    pub code_quality_metrics: CodeQualityMetrics,
}

#[derive(Debug, Serialize)]
pub struct LanguageStats {
    pub language: String,
    pub file_count: u64,
    pub line_count: u64,
    pub percentage: f64,
    pub color: String,
}

#[derive(Debug, Serialize)]
pub struct GenerationTimePoint {
    pub timestamp: DateTime<Utc>,
    pub files_generated: u64,
    pub lines_generated: u64,
}

#[derive(Debug, Serialize)]
pub struct CodeQualityMetrics {
    pub avg_file_size_kb: f64,
    pub avg_complexity_score: f64,
    pub estimated_bug_rate: f64,
    pub readability_score: f64,
}

pub fn routes() -> Router<Arc<dyn Database>> {
    Router::new()
        .route("/productivity", get(get_productivity_metrics))
        .route("/costs", get(get_cost_analytics))
        .route("/efficiency", get(get_efficiency_metrics))
        .route("/trends", get(get_trend_analysis))
        .route("/dashboard/kpis", get(get_dashboard_kpis))
        .route("/dashboard/token-trend", get(get_token_trend))
        .route("/dashboard/tool-usage", get(get_tool_usage))
        .route("/dashboard/usage-heatmap", get(get_usage_heatmap))
        .route("/advanced/model-costs", get(get_model_cost_comparison))
        .route("/advanced/budget-progress", get(get_budget_progress))
        .route("/advanced/tool-efficiency", get(get_advanced_tool_efficiency))
        .route("/advanced/session-duration", get(get_session_duration_distribution))
        .route("/advanced/code-generation", get(get_code_generation_stats))
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

// New dashboard endpoints
// GET /api/analytics/dashboard/kpis - Dashboard KPI summary
async fn get_dashboard_kpis(
    State(_db): State<Arc<dyn Database>>,
    Query(params): Query<AnalyticsQuery>,
) -> ApiResult<impl IntoResponse> {
    let range = params.range.as_deref().unwrap_or("24h");
    
    // TODO: Implement actual KPI calculations from database
    let kpis = DashboardKPIs {
        today_sessions: 24,
        today_sessions_change: 12.5, // +12.5% from yesterday
        total_tokens: 145_892,
        total_tokens_change: -3.2, // -3.2% from previous period
        total_cost: 23.47,
        total_cost_change: 8.1, // +8.1% from previous period
        lines_of_code: 1_247,
        lines_of_code_change: 15.8, // +15.8% from previous period
        period: range.to_string(),
    };

    Ok(Json(ApiResponse::success(kpis)))
}

// GET /api/analytics/dashboard/token-trend - Token usage trend over time
async fn get_token_trend(
    State(_db): State<Arc<dyn Database>>,
    Query(params): Query<AnalyticsQuery>,
) -> ApiResult<impl IntoResponse> {
    let (start_time, end_time) = parse_time_range(&params)?;
    let range = params.range.as_deref().unwrap_or("24h");
    
    let mut data_points = Vec::new();
    let duration = end_time - start_time;
    let num_points = match range {
        "24h" => 24,
        "7d" => 7 * 4, // 4 points per day
        "30d" => 30,
        _ => 24,
    };
    
    for i in 0..num_points {
        let timestamp = start_time + duration * i as i32 / num_points as i32;
        let base_input = 1000 + (i * 50) as u64;
        let base_output = 600 + (i * 30) as u64;
        let cache_creation = 50 + (i * 5) as u64;
        let cache_read = 200 + (i * 10) as u64;
        
        data_points.push(TokenTrendPoint {
            timestamp,
            input_tokens: base_input,
            output_tokens: base_output,
            cache_creation_tokens: cache_creation,
            cache_read_tokens: cache_read,
            total_tokens: base_input + base_output + cache_creation + cache_read,
        });
    }
    
    let trend_data = TokenTrendData {
        range: range.to_string(),
        data_points,
    };

    Ok(Json(ApiResponse::success(trend_data)))
}

// GET /api/analytics/dashboard/tool-usage - Tool usage statistics
async fn get_tool_usage(
    State(_db): State<Arc<dyn Database>>,
    Query(_params): Query<AnalyticsQuery>,
) -> ApiResult<impl IntoResponse> {
    // TODO: Implement actual tool usage queries from database
    let tools = vec![
        ToolUsageStats {
            tool_name: "Edit".to_string(),
            usage_count: 456,
            success_rate: 97.4,
            avg_duration_ms: 1_250.0,
            percentage: 35.2,
            color: "#8b5cf6".to_string(),
        },
        ToolUsageStats {
            tool_name: "Read".to_string(),
            usage_count: 324,
            success_rate: 99.1,
            avg_duration_ms: 580.0,
            percentage: 25.0,
            color: "#06b6d4".to_string(),
        },
        ToolUsageStats {
            tool_name: "Bash".to_string(),
            usage_count: 189,
            success_rate: 94.3,
            avg_duration_ms: 2_840.0,
            percentage: 14.6,
            color: "#10b981".to_string(),
        },
        ToolUsageStats {
            tool_name: "Write".to_string(),
            usage_count: 156,
            success_rate: 96.8,
            avg_duration_ms: 1_890.0,
            percentage: 12.0,
            color: "#f59e0b".to_string(),
        },
        ToolUsageStats {
            tool_name: "Grep".to_string(),
            usage_count: 123,
            success_rate: 98.4,
            avg_duration_ms: 750.0,
            percentage: 9.5,
            color: "#ef4444".to_string(),
        },
        ToolUsageStats {
            tool_name: "Other".to_string(),
            usage_count: 48,
            success_rate: 92.1,
            avg_duration_ms: 1_340.0,
            percentage: 3.7,
            color: "#6b7280".to_string(),
        },
    ];
    
    let total_calls = tools.iter().map(|t| t.usage_count).sum();
    
    let usage_data = ToolUsageData {
        total_tool_calls: total_calls,
        tools,
    };

    Ok(Json(ApiResponse::success(usage_data)))
}

// GET /api/analytics/dashboard/usage-heatmap - Usage activity heatmap
async fn get_usage_heatmap(
    State(_db): State<Arc<dyn Database>>,
    Query(_params): Query<AnalyticsQuery>,
) -> ApiResult<impl IntoResponse> {
    // TODO: Implement actual heatmap data from database
    let mut heatmap = Vec::new();
    
    // Generate 7 days x 24 hours heatmap
    for day in 0..7 {
        for hour in 0..24 {
            // Mock intensity based on typical work patterns
            let intensity = match (day, hour) {
                // Weekend
                (0, _) | (6, _) => 0.1 + ((hour * day + 7) as f64 % 5.0) * 0.04,
                // Weekday business hours (9-18)
                (_, h) if h >= 9 && h <= 18 => 0.4 + ((h * day + 13) as f64 % 10.0) * 0.06,
                // Weekday evening
                (_, h) if h >= 19 && h <= 23 => 0.2 + ((h + day * 3) as f64 % 7.0) * 0.057,
                // Night/early morning
                _ => ((hour + day * 2) as f64 % 11.0) * 0.027,
            };
            
            heatmap.push(HeatmapCell {
                hour: hour as u8,
                day_of_week: day,
                intensity: intensity.min(1.0),
                session_count: ((intensity * 10.0) as u64).max(1),
                token_count: ((intensity * 5000.0) as u64).max(100),
            });
        }
    }
    
    let heatmap_data = UsageHeatmapData {
        timezone: "UTC".to_string(),
        heatmap,
    };

    Ok(Json(ApiResponse::success(heatmap_data)))
}

// Advanced analytics endpoints for the analytics page

// GET /api/analytics/advanced/model-costs - Model cost comparison
async fn get_model_cost_comparison(
    State(_db): State<Arc<dyn Database>>,
    Query(params): Query<AnalyticsQuery>,
) -> ApiResult<impl IntoResponse> {
    let range = params.range.as_deref().unwrap_or("30d");
    
    let models = vec![
        ModelCostComparisonItem {
            model_name: "claude-3-5-sonnet-20241022".to_string(),
            cost_per_session: 1.85,
            total_sessions: 145,
            total_cost: 268.25,
            avg_input_tokens: 2847,
            avg_output_tokens: 1593,
            efficiency_score: 0.059, // cost per token
            color: "#8b5cf6".to_string(),
        },
        ModelCostComparisonItem {
            model_name: "claude-3-haiku-20240307".to_string(),
            cost_per_session: 0.42,
            total_sessions: 89,
            total_cost: 37.38,
            avg_input_tokens: 1245,
            avg_output_tokens: 843,
            efficiency_score: 0.018,
            color: "#06b6d4".to_string(),
        },
        ModelCostComparisonItem {
            model_name: "claude-3-opus-20240229".to_string(),
            cost_per_session: 3.24,
            total_sessions: 23,
            total_cost: 74.52,
            avg_input_tokens: 3456,
            avg_output_tokens: 2134,
            efficiency_score: 0.133,
            color: "#f59e0b".to_string(),
        },
    ];
    
    let total_cost = models.iter().map(|m| m.total_cost).sum();
    
    let comparison = ModelCostComparison {
        models,
        total_cost,
        period: range.to_string(),
    };

    Ok(Json(ApiResponse::success(comparison)))
}

// GET /api/analytics/advanced/budget-progress - Budget tracking
async fn get_budget_progress(
    State(_db): State<Arc<dyn Database>>,
    Query(_params): Query<AnalyticsQuery>,
) -> ApiResult<impl IntoResponse> {
    let current_cost = 380.15;
    let budget = 500.0;
    let days_in_month = 30;
    let days_passed = 18;
    let days_remaining = days_in_month - days_passed;
    
    // Generate daily breakdown for the current month
    let mut daily_breakdown = Vec::new();
    let now = Utc::now();
    
    for i in 0..days_passed {
        let date = now - Duration::days(days_passed as i64 - i as i64);
        let base_cost = 15.0 + (i as f64 * 1.2) + ((i * 7) % 13) as f64 * 0.8;
        daily_breakdown.push(DailyCostBreakdown {
            date,
            cost: base_cost,
            sessions: 3 + (i % 8) as u64,
            tokens: ((base_cost * 1500.0) as u64),
        });
    }
    
    let projected_cost = current_cost / days_passed as f64 * days_in_month as f64;
    
    let progress = BudgetProgressData {
        current_month_cost: current_cost,
        monthly_budget: budget,
        percentage_used: (current_cost / budget * 100.0),
        days_remaining: days_remaining as u32,
        projected_month_end_cost: projected_cost,
        is_over_budget: projected_cost > budget,
        daily_breakdown,
    };

    Ok(Json(ApiResponse::success(progress)))
}

// GET /api/analytics/advanced/tool-efficiency - Advanced tool efficiency analysis
async fn get_advanced_tool_efficiency(
    State(_db): State<Arc<dyn Database>>,
    Query(params): Query<AnalyticsQuery>,
) -> ApiResult<impl IntoResponse> {
    let (start_time, end_time) = parse_time_range(&params)?;
    
    let tools = vec![
        AdvancedToolStats {
            tool_name: "Edit".to_string(),
            usage_count: 456,
            success_rate: 97.4,
            avg_duration_ms: 1_250.0,
            median_duration_ms: 980.0,
            efficiency_score: 9.2,
            time_saved_estimate_hours: 23.4,
            cost_per_use: 0.085,
            trend: TrendDirection::Increasing(5.2),
        },
        AdvancedToolStats {
            tool_name: "Read".to_string(),
            usage_count: 324,
            success_rate: 99.1,
            avg_duration_ms: 580.0,
            median_duration_ms: 450.0,
            efficiency_score: 9.8,
            time_saved_estimate_hours: 45.2,
            cost_per_use: 0.032,
            trend: TrendDirection::Increasing(2.1),
        },
        AdvancedToolStats {
            tool_name: "Bash".to_string(),
            usage_count: 189,
            success_rate: 94.3,
            avg_duration_ms: 2_840.0,
            median_duration_ms: 1_950.0,
            efficiency_score: 7.6,
            time_saved_estimate_hours: 18.7,
            cost_per_use: 0.145,
            trend: TrendDirection::Stable,
        },
        AdvancedToolStats {
            tool_name: "Write".to_string(),
            usage_count: 156,
            success_rate: 96.8,
            avg_duration_ms: 1_890.0,
            median_duration_ms: 1_450.0,
            efficiency_score: 8.4,
            time_saved_estimate_hours: 12.3,
            cost_per_use: 0.098,
            trend: TrendDirection::Decreasing(1.8),
        },
    ];
    
    // Generate efficiency over time
    let mut efficiency_points = Vec::new();
    let duration = end_time - start_time;
    let num_points = 20;
    
    for i in 0..num_points {
        let timestamp = start_time + duration * i as i32 / num_points as i32;
        efficiency_points.push(EfficiencyTimePoint {
            timestamp,
            overall_score: 8.5 + ((i * 3) % 7) as f64 * 0.2 - 1.0,
            top_tool_score: 9.8 + ((i * 5) % 3) as f64 * 0.15 - 0.4,
        });
    }
    
    let overall_score = tools.iter().map(|t| t.efficiency_score * t.usage_count as f64)
        .sum::<f64>() / tools.iter().map(|t| t.usage_count).sum::<u64>() as f64;
    
    let efficiency = AdvancedToolEfficiency {
        overall_efficiency_score: overall_score,
        tools,
        efficiency_over_time: efficiency_points,
    };

    Ok(Json(ApiResponse::success(efficiency)))
}

// GET /api/analytics/advanced/session-duration - Session duration distribution
async fn get_session_duration_distribution(
    State(_db): State<Arc<dyn Database>>,
    Query(params): Query<AnalyticsQuery>,
) -> ApiResult<impl IntoResponse> {
    let (start_time, end_time) = parse_time_range(&params)?;
    
    let buckets = vec![
        DurationBucket {
            min_minutes: 0,
            max_minutes: 5,
            session_count: 23,
            percentage: 15.4,
            label: "0-5 min".to_string(),
        },
        DurationBucket {
            min_minutes: 5,
            max_minutes: 15,
            session_count: 45,
            percentage: 30.2,
            label: "5-15 min".to_string(),
        },
        DurationBucket {
            min_minutes: 15,
            max_minutes: 30,
            session_count: 38,
            percentage: 25.5,
            label: "15-30 min".to_string(),
        },
        DurationBucket {
            min_minutes: 30,
            max_minutes: 60,
            session_count: 28,
            percentage: 18.8,
            label: "30-60 min".to_string(),
        },
        DurationBucket {
            min_minutes: 60,
            max_minutes: 120,
            session_count: 12,
            percentage: 8.1,
            label: "1-2 hours".to_string(),
        },
        DurationBucket {
            min_minutes: 120,
            max_minutes: u32::MAX,
            session_count: 3,
            percentage: 2.0,
            label: "2+ hours".to_string(),
        },
    ];
    
    let total_sessions = buckets.iter().map(|b| b.session_count).sum();
    
    // Generate duration over time
    let mut duration_points = Vec::new();
    let duration = end_time - start_time;
    let num_points = 15;
    
    for i in 0..num_points {
        let timestamp = start_time + duration * i as i32 / num_points as i32;
        duration_points.push(DurationTimePoint {
            timestamp,
            avg_duration_minutes: 22.5 + ((i * 7) % 11) as f64 * 2.3,
            session_count: 8 + (i % 6) as u64,
        });
    }
    
    let distribution = SessionDurationDistribution {
        total_sessions,
        avg_duration_minutes: 24.7,
        median_duration_minutes: 18.3,
        distribution_buckets: buckets,
        duration_over_time: duration_points,
    };

    Ok(Json(ApiResponse::success(distribution)))
}

// GET /api/analytics/advanced/code-generation - Code generation statistics
async fn get_code_generation_stats(
    State(_db): State<Arc<dyn Database>>,
    Query(params): Query<AnalyticsQuery>,
) -> ApiResult<impl IntoResponse> {
    let (start_time, end_time) = parse_time_range(&params)?;
    
    let languages = vec![
        LanguageStats {
            language: "TypeScript".to_string(),
            file_count: 125,
            line_count: 8947,
            percentage: 35.2,
            color: "#3178c6".to_string(),
        },
        LanguageStats {
            language: "Rust".to_string(),
            file_count: 78,
            line_count: 6234,
            percentage: 24.5,
            color: "#dea584".to_string(),
        },
        LanguageStats {
            language: "Python".to_string(),
            file_count: 89,
            line_count: 5432,
            percentage: 21.4,
            color: "#3776ab".to_string(),
        },
        LanguageStats {
            language: "JavaScript".to_string(),
            file_count: 45,
            line_count: 3245,
            percentage: 12.8,
            color: "#f7df1e".to_string(),
        },
        LanguageStats {
            language: "Other".to_string(),
            file_count: 23,
            line_count: 1567,
            percentage: 6.1,
            color: "#6b7280".to_string(),
        },
    ];
    
    let total_files = languages.iter().map(|l| l.file_count).sum();
    let total_lines = languages.iter().map(|l| l.line_count).sum();
    
    // Generate generation over time
    let mut generation_points = Vec::new();
    let duration = end_time - start_time;
    let num_points = 12;
    
    for i in 0..num_points {
        let timestamp = start_time + duration * i as i32 / num_points as i32;
        generation_points.push(GenerationTimePoint {
            timestamp,
            files_generated: 5 + ((i * 3) % 8) as u64,
            lines_generated: 234 + ((i * 47) % 156) as u64,
        });
    }
    
    let stats = CodeGenerationStats {
        total_code_files_generated: total_files,
        total_lines_generated: total_lines,
        avg_lines_per_file: total_lines as f64 / total_files as f64,
        most_generated_languages: languages,
        generation_over_time: generation_points,
        code_quality_metrics: CodeQualityMetrics {
            avg_file_size_kb: 2.8,
            avg_complexity_score: 6.4,
            estimated_bug_rate: 0.024,
            readability_score: 8.7,
        },
    };

    Ok(Json(ApiResponse::success(stats)))
}