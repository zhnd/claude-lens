use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde_json;
use sqlx::{sqlite::SqlitePool, Row};
use std::{collections::HashMap, sync::Arc};
use uuid::Uuid;

use super::{
    Database, DatabaseError, LogRecord, MetricRecord, SessionRecord, TraceRecord,
};

pub struct SqliteDatabase {
    pool: SqlitePool,
}

impl SqliteDatabase {
    pub async fn new(database_url: &str) -> Result<Self, DatabaseError> {
        let pool = SqlitePool::connect(database_url)
            .await
            .map_err(|e| DatabaseError::Connection(e.to_string()))?;

        Ok(Self { pool })
    }

    pub async fn migrate(&self) -> Result<(), DatabaseError> {
        // Run the initial migration manually for now
        // TODO: Use sqlx::migrate!() once migration files are properly set up
        let migration_sql = r#"
        -- Claude Scope Database Schema
        -- Initial migration for storing OpenTelemetry data

        -- Sessions table: tracks Claude Code sessions
        CREATE TABLE IF NOT EXISTS sessions (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            start_time DATETIME NOT NULL,
            end_time DATETIME NULL,
            command_count INTEGER NOT NULL DEFAULT 0,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        );

        CREATE INDEX IF NOT EXISTS idx_sessions_user_id ON sessions(user_id);
        CREATE INDEX IF NOT EXISTS idx_sessions_start_time ON sessions(start_time);

        -- Metrics table: stores OpenTelemetry metrics data
        CREATE TABLE IF NOT EXISTS metrics (
            id TEXT PRIMARY KEY,
            session_id TEXT NULL,
            name TEXT NOT NULL,
            timestamp DATETIME NOT NULL,
            value REAL NOT NULL,
            labels TEXT NOT NULL, -- JSON string of key-value pairs
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
        );

        CREATE INDEX IF NOT EXISTS idx_metrics_name ON metrics(name);
        CREATE INDEX IF NOT EXISTS idx_metrics_timestamp ON metrics(timestamp);
        CREATE INDEX IF NOT EXISTS idx_metrics_session_id ON metrics(session_id);

        -- Traces table: stores OpenTelemetry trace/span data
        CREATE TABLE IF NOT EXISTS traces (
            id TEXT PRIMARY KEY,
            session_id TEXT NULL,
            trace_id TEXT NOT NULL,
            span_id TEXT NOT NULL,
            parent_span_id TEXT NULL,
            name TEXT NOT NULL,
            start_time DATETIME NOT NULL,
            end_time DATETIME NOT NULL,
            duration_ns INTEGER NOT NULL,
            attributes TEXT NOT NULL, -- JSON string of key-value pairs
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
        );

        CREATE INDEX IF NOT EXISTS idx_traces_trace_id ON traces(trace_id);
        CREATE INDEX IF NOT EXISTS idx_traces_span_id ON traces(span_id);
        CREATE INDEX IF NOT EXISTS idx_traces_start_time ON traces(start_time);
        CREATE INDEX IF NOT EXISTS idx_traces_session_id ON traces(session_id);

        -- Logs table: stores OpenTelemetry log data
        CREATE TABLE IF NOT EXISTS logs (
            id TEXT PRIMARY KEY,
            session_id TEXT NULL,
            timestamp DATETIME NOT NULL,
            level TEXT NOT NULL,
            message TEXT NOT NULL,
            attributes TEXT NOT NULL, -- JSON string of key-value pairs
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
        );

        CREATE INDEX IF NOT EXISTS idx_logs_timestamp ON logs(timestamp);
        CREATE INDEX IF NOT EXISTS idx_logs_level ON logs(level);
        CREATE INDEX IF NOT EXISTS idx_logs_session_id ON logs(session_id);
        "#;

        sqlx::query(migration_sql)
            .execute(&self.pool)
            .await
            .map_err(|e| DatabaseError::Migration(e.to_string()))?;
        
        Ok(())
    }
}

#[async_trait]
impl Database for SqliteDatabase {
    async fn create_session(&self, user_id: &str) -> Result<Uuid, DatabaseError> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO sessions (id, user_id, start_time, command_count, created_at, updated_at)
            VALUES (?1, ?2, ?3, 0, ?4, ?5)
            "#
        )
        .bind(id.to_string())
        .bind(user_id)
        .bind(now)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?;

        Ok(id)
    }

    async fn get_session(&self, session_id: Uuid) -> Result<Option<SessionRecord>, DatabaseError> {
        let row = sqlx::query("SELECT id, user_id, start_time, end_time, command_count, created_at, updated_at FROM sessions WHERE id = ?1")
            .bind(session_id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DatabaseError::Query(e.to_string()))?;

        match row {
            Some(row) => Ok(Some(SessionRecord {
                id: Uuid::parse_str(row.get("id"))
                    .map_err(|e| DatabaseError::InvalidData(e.to_string()))?,
                user_id: row.get("user_id"),
                start_time: row.get("start_time"),
                end_time: row.get("end_time"),
                command_count: row.get::<i64, _>("command_count") as u64,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })),
            None => Ok(None),
        }
    }

    async fn update_session(
        &self,
        session_id: Uuid,
        end_time: Option<DateTime<Utc>>,
    ) -> Result<(), DatabaseError> {
        let now = Utc::now();

        sqlx::query("UPDATE sessions SET end_time = ?1, updated_at = ?2 WHERE id = ?3")
            .bind(end_time)
            .bind(now)
            .bind(session_id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| DatabaseError::Query(e.to_string()))?;

        Ok(())
    }

    async fn list_sessions(
        &self,
        user_id: Option<&str>,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<SessionRecord>, DatabaseError> {
        let rows = if let Some(uid) = user_id {
            sqlx::query("SELECT id, user_id, start_time, end_time, command_count, created_at, updated_at FROM sessions WHERE user_id = ?1 ORDER BY start_time DESC LIMIT ?2 OFFSET ?3")
                .bind(uid)
                .bind(limit as i64)
                .bind(offset as i64)
                .fetch_all(&self.pool)
                .await
        } else {
            sqlx::query("SELECT id, user_id, start_time, end_time, command_count, created_at, updated_at FROM sessions ORDER BY start_time DESC LIMIT ?1 OFFSET ?2")
                .bind(limit as i64)
                .bind(offset as i64)
                .fetch_all(&self.pool)
                .await
        };

        let rows = rows.map_err(|e| DatabaseError::Query(e.to_string()))?;

        let mut sessions = Vec::new();
        for row in rows {
            sessions.push(SessionRecord {
                id: Uuid::parse_str(row.get("id"))
                    .map_err(|e| DatabaseError::InvalidData(e.to_string()))?,
                user_id: row.get("user_id"),
                start_time: row.get("start_time"),
                end_time: row.get("end_time"),
                command_count: row.get::<i64, _>("command_count") as u64,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(sessions)
    }

    async fn store_metric(&self, metric: &MetricRecord) -> Result<(), DatabaseError> {
        let labels_json = serde_json::to_string(&metric.labels)
            .map_err(|e| DatabaseError::InvalidData(e.to_string()))?;

        sqlx::query(
            r#"
            INSERT INTO metrics (id, session_id, name, timestamp, value, labels, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#
        )
        .bind(metric.id.to_string())
        .bind(metric.session_id.map(|id| id.to_string()))
        .bind(&metric.name)
        .bind(metric.timestamp)
        .bind(metric.value)
        .bind(labels_json)
        .bind(metric.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?;

        Ok(())
    }

    async fn get_metrics(
        &self,
        _start_time: Option<DateTime<Utc>>,
        _end_time: Option<DateTime<Utc>>,
        _metric_name: Option<&str>,
    ) -> Result<Vec<MetricRecord>, DatabaseError> {
        // This is a simplified query - in practice, you'd want to build dynamic WHERE clauses
        let rows = sqlx::query("SELECT id, session_id, name, timestamp, value, labels, created_at FROM metrics ORDER BY timestamp DESC")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DatabaseError::Query(e.to_string()))?;

        let mut metrics = Vec::new();
        for row in rows {
            let labels_str: String = row.get("labels");
            let labels: HashMap<String, String> = serde_json::from_str(&labels_str)
                .map_err(|e| DatabaseError::InvalidData(e.to_string()))?;

            metrics.push(MetricRecord {
                id: Uuid::parse_str(row.get("id"))
                    .map_err(|e| DatabaseError::InvalidData(e.to_string()))?,
                session_id: row.get::<Option<String>, _>("session_id")
                    .map(|s| Uuid::parse_str(&s))
                    .transpose()
                    .map_err(|e| DatabaseError::InvalidData(e.to_string()))?,
                name: row.get("name"),
                timestamp: row.get("timestamp"),
                value: row.get("value"),
                labels,
                created_at: row.get("created_at"),
            });
        }

        Ok(metrics)
    }

    async fn store_trace(&self, trace: &TraceRecord) -> Result<(), DatabaseError> {
        let attributes_json = serde_json::to_string(&trace.attributes)
            .map_err(|e| DatabaseError::InvalidData(e.to_string()))?;

        sqlx::query(
            r#"
            INSERT INTO traces (id, session_id, trace_id, span_id, parent_span_id, name, start_time, end_time, duration_ns, attributes, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
            "#
        )
        .bind(trace.id.to_string())
        .bind(trace.session_id.map(|id| id.to_string()))
        .bind(&trace.trace_id)
        .bind(&trace.span_id)
        .bind(trace.parent_span_id.as_ref())
        .bind(&trace.name)
        .bind(trace.start_time)
        .bind(trace.end_time)
        .bind(trace.duration_ns as i64)
        .bind(attributes_json)
        .bind(trace.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?;

        Ok(())
    }

    async fn get_traces(
        &self,
        _start_time: Option<DateTime<Utc>>,
        _end_time: Option<DateTime<Utc>>,
        _trace_id: Option<&str>,
    ) -> Result<Vec<TraceRecord>, DatabaseError> {
        // TODO: Implement trace retrieval with filtering
        Ok(vec![])
    }

    async fn store_log(&self, log: &LogRecord) -> Result<(), DatabaseError> {
        let attributes_json = serde_json::to_string(&log.attributes)
            .map_err(|e| DatabaseError::InvalidData(e.to_string()))?;

        sqlx::query(
            r#"
            INSERT INTO logs (id, session_id, timestamp, level, message, attributes, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#
        )
        .bind(log.id.to_string())
        .bind(log.session_id.map(|id| id.to_string()))
        .bind(log.timestamp)
        .bind(&log.level)
        .bind(&log.message)
        .bind(attributes_json)
        .bind(log.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| DatabaseError::Query(e.to_string()))?;

        Ok(())
    }

    async fn get_logs(
        &self,
        _start_time: Option<DateTime<Utc>>,
        _end_time: Option<DateTime<Utc>>,
        _level: Option<&str>,
    ) -> Result<Vec<LogRecord>, DatabaseError> {
        // TODO: Implement log retrieval with filtering
        Ok(vec![])
    }
}

pub async fn init_database(database_path: &str) -> Result<Arc<dyn Database>, DatabaseError> {
    use std::path::Path;
    
    // Ensure the parent directory exists
    if let Some(parent) = Path::new(database_path).parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)
                .map_err(|e| DatabaseError::Connection(format!(
                    "Failed to create database directory {}: {}", 
                    parent.display(), 
                    e
                )))?;
        }
    }
    
    let database_url = format!("sqlite:{}?mode=rwc", database_path);
    tracing::info!("Connecting to database at: {}", database_path);
    
    let db = SqliteDatabase::new(&database_url).await?;
    tracing::info!("Running database migrations...");
    db.migrate().await?;
    tracing::info!("Database initialized successfully");
    
    Ok(Arc::new(db))
}