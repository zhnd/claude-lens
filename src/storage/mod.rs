pub mod sqlite;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

#[async_trait]
pub trait Database: Send + Sync {
    // Session operations
    async fn create_session(&self, user_id: &str) -> Result<Uuid, DatabaseError>;
    async fn get_session(&self, session_id: Uuid) -> Result<Option<SessionRecord>, DatabaseError>;
    async fn update_session(&self, session_id: Uuid, end_time: Option<DateTime<Utc>>) -> Result<(), DatabaseError>;
    async fn list_sessions(&self, user_id: Option<&str>, limit: u32, offset: u32) -> Result<Vec<SessionRecord>, DatabaseError>;

    // Metrics operations
    async fn store_metric(&self, metric: &MetricRecord) -> Result<(), DatabaseError>;
    async fn get_metrics(
        &self,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        metric_name: Option<&str>,
    ) -> Result<Vec<MetricRecord>, DatabaseError>;

    // Trace operations
    async fn store_trace(&self, trace: &TraceRecord) -> Result<(), DatabaseError>;
    async fn get_traces(
        &self,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        trace_id: Option<&str>,
    ) -> Result<Vec<TraceRecord>, DatabaseError>;

    // Log operations
    async fn store_log(&self, log: &LogRecord) -> Result<(), DatabaseError>;
    async fn get_logs(
        &self,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        level: Option<&str>,
    ) -> Result<Vec<LogRecord>, DatabaseError>;
}

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Database connection error: {0}")]
    Connection(String),
    #[error("Database query error: {0}")]
    Query(String),
    #[error("Database migration error: {0}")]
    Migration(String),
    #[error("Record not found")]
    NotFound,
    #[error("Invalid data: {0}")]
    InvalidData(String),
}

#[derive(Debug, Clone)]
pub struct SessionRecord {
    pub id: Uuid,
    pub user_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub command_count: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct MetricRecord {
    pub id: Uuid,
    pub session_id: Option<Uuid>,
    pub name: String,
    pub timestamp: DateTime<Utc>,
    pub value: f64,
    pub labels: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct TraceRecord {
    pub id: Uuid,
    pub session_id: Option<Uuid>,
    pub trace_id: String,
    pub span_id: String,
    pub parent_span_id: Option<String>,
    pub name: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub duration_ns: u64,
    pub attributes: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct LogRecord {
    pub id: Uuid,
    pub session_id: Option<Uuid>,
    pub timestamp: DateTime<Utc>,
    pub level: String,
    pub message: String,
    pub attributes: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
}