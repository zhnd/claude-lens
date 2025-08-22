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