-- Enhanced metrics schema for Datadog-style monitoring
-- Migration 002: Add user identification and enhanced metric types

-- Add user identification columns to metrics table
ALTER TABLE metrics ADD COLUMN metric_type TEXT;
ALTER TABLE metrics ADD COLUMN user_id TEXT;
ALTER TABLE metrics ADD COLUMN user_email TEXT;
ALTER TABLE metrics ADD COLUMN organization_id TEXT;
ALTER TABLE metrics ADD COLUMN version TEXT;
ALTER TABLE metrics ADD COLUMN host TEXT;
ALTER TABLE metrics ADD COLUMN service TEXT;

-- Add user identification columns to sessions table
ALTER TABLE sessions ADD COLUMN user_email TEXT;
ALTER TABLE sessions ADD COLUMN organization_id TEXT;
ALTER TABLE sessions ADD COLUMN host TEXT;
ALTER TABLE sessions ADD COLUMN version TEXT;

-- Create indexes for user-based queries
CREATE INDEX IF NOT EXISTS idx_metrics_user_id ON metrics(user_id);
CREATE INDEX IF NOT EXISTS idx_metrics_user_email ON metrics(user_email);
CREATE INDEX IF NOT EXISTS idx_metrics_organization_id ON metrics(organization_id);
CREATE INDEX IF NOT EXISTS idx_metrics_metric_type ON metrics(metric_type);

CREATE INDEX IF NOT EXISTS idx_sessions_user_email ON sessions(user_email);
CREATE INDEX IF NOT EXISTS idx_sessions_organization_id ON sessions(organization_id);

-- Productivity events table for tracking commits, PRs, and code changes
CREATE TABLE IF NOT EXISTS productivity_events (
    id TEXT PRIMARY KEY,
    session_id TEXT,
    event_type TEXT NOT NULL, -- 'commit', 'pull_request', 'file_edit'
    repository TEXT,
    files_changed INTEGER DEFAULT 0,
    lines_added INTEGER DEFAULT 0,
    lines_removed INTEGER DEFAULT 0,
    commit_hash TEXT,
    branch_name TEXT,
    pr_number INTEGER,
    timestamp DATETIME NOT NULL,
    metadata TEXT, -- JSON for additional data
    user_id TEXT,
    user_email TEXT,
    organization_id TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_productivity_events_event_type ON productivity_events(event_type);
CREATE INDEX IF NOT EXISTS idx_productivity_events_timestamp ON productivity_events(timestamp);
CREATE INDEX IF NOT EXISTS idx_productivity_events_session_id ON productivity_events(session_id);
CREATE INDEX IF NOT EXISTS idx_productivity_events_user_email ON productivity_events(user_email);

-- Cost tracking table for token usage and financial metrics
CREATE TABLE IF NOT EXISTS cost_tracking (
    id TEXT PRIMARY KEY,
    session_id TEXT,
    model_name TEXT NOT NULL,
    input_tokens INTEGER DEFAULT 0,
    output_tokens INTEGER DEFAULT 0,
    cache_creation_tokens INTEGER DEFAULT 0,
    cache_read_tokens INTEGER DEFAULT 0,
    cost_usd REAL NOT NULL,
    timestamp DATETIME NOT NULL,
    user_id TEXT,
    user_email TEXT,
    organization_id TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_cost_tracking_timestamp ON cost_tracking(timestamp);
CREATE INDEX IF NOT EXISTS idx_cost_tracking_session_id ON cost_tracking(session_id);
CREATE INDEX IF NOT EXISTS idx_cost_tracking_user_email ON cost_tracking(user_email);
CREATE INDEX IF NOT EXISTS idx_cost_tracking_model_name ON cost_tracking(model_name);

-- Tool usage tracking table
CREATE TABLE IF NOT EXISTS tool_usage (
    id TEXT PRIMARY KEY,
    session_id TEXT,
    tool_name TEXT NOT NULL,
    invocation_count INTEGER NOT NULL DEFAULT 1,
    total_duration_ms INTEGER DEFAULT 0,
    success_count INTEGER DEFAULT 0,
    error_count INTEGER DEFAULT 0,
    timestamp DATETIME NOT NULL,
    user_id TEXT,
    user_email TEXT,
    organization_id TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_tool_usage_tool_name ON tool_usage(tool_name);
CREATE INDEX IF NOT EXISTS idx_tool_usage_timestamp ON tool_usage(timestamp);
CREATE INDEX IF NOT EXISTS idx_tool_usage_session_id ON tool_usage(session_id);
CREATE INDEX IF NOT EXISTS idx_tool_usage_user_email ON tool_usage(user_email);

-- Session summary materialized view for performance
CREATE TABLE IF NOT EXISTS session_summaries (
    session_id TEXT PRIMARY KEY,
    user_id TEXT,
    user_email TEXT,
    organization_id TEXT,
    start_time DATETIME NOT NULL,
    end_time DATETIME,
    duration_seconds INTEGER,
    total_input_tokens INTEGER DEFAULT 0,
    total_output_tokens INTEGER DEFAULT 0,
    total_cache_creation_tokens INTEGER DEFAULT 0,
    total_cache_read_tokens INTEGER DEFAULT 0,
    total_cost_usd REAL DEFAULT 0.0,
    commit_count INTEGER DEFAULT 0,
    pull_request_count INTEGER DEFAULT 0,
    lines_added INTEGER DEFAULT 0,
    lines_removed INTEGER DEFAULT 0,
    tool_invocations INTEGER DEFAULT 0,
    error_count INTEGER DEFAULT 0,
    last_updated DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_session_summaries_user_email ON session_summaries(user_email);
CREATE INDEX IF NOT EXISTS idx_session_summaries_start_time ON session_summaries(start_time);
CREATE INDEX IF NOT EXISTS idx_session_summaries_total_cost_usd ON session_summaries(total_cost_usd);

-- Triggers to update session summaries automatically
CREATE TRIGGER IF NOT EXISTS update_session_summary_from_cost
AFTER INSERT ON cost_tracking
BEGIN
    INSERT OR REPLACE INTO session_summaries (
        session_id, user_id, user_email, organization_id, start_time, end_time,
        duration_seconds, total_input_tokens, total_output_tokens, 
        total_cache_creation_tokens, total_cache_read_tokens, total_cost_usd,
        commit_count, pull_request_count, lines_added, lines_removed,
        tool_invocations, error_count, last_updated
    )
    SELECT 
        COALESCE(NEW.session_id, ss.session_id),
        COALESCE(NEW.user_id, ss.user_id),
        COALESCE(NEW.user_email, ss.user_email),
        COALESCE(NEW.organization_id, ss.organization_id),
        COALESCE(s.start_time, ss.start_time),
        COALESCE(s.end_time, ss.end_time),
        CASE WHEN s.end_time IS NOT NULL THEN 
            CAST((julianday(s.end_time) - julianday(s.start_time)) * 86400 AS INTEGER)
        ELSE ss.duration_seconds END,
        COALESCE(ss.total_input_tokens, 0) + NEW.input_tokens,
        COALESCE(ss.total_output_tokens, 0) + NEW.output_tokens,
        COALESCE(ss.total_cache_creation_tokens, 0) + NEW.cache_creation_tokens,
        COALESCE(ss.total_cache_read_tokens, 0) + NEW.cache_read_tokens,
        COALESCE(ss.total_cost_usd, 0.0) + NEW.cost_usd,
        COALESCE(ss.commit_count, 0),
        COALESCE(ss.pull_request_count, 0),
        COALESCE(ss.lines_added, 0),
        COALESCE(ss.lines_removed, 0),
        COALESCE(ss.tool_invocations, 0),
        COALESCE(ss.error_count, 0),
        NEW.created_at
    FROM sessions s
    LEFT JOIN session_summaries ss ON ss.session_id = NEW.session_id
    WHERE s.id = NEW.session_id;
END;

CREATE TRIGGER IF NOT EXISTS update_session_summary_from_productivity
AFTER INSERT ON productivity_events
BEGIN
    INSERT OR REPLACE INTO session_summaries (
        session_id, user_id, user_email, organization_id, start_time, end_time,
        duration_seconds, total_input_tokens, total_output_tokens, 
        total_cache_creation_tokens, total_cache_read_tokens, total_cost_usd,
        commit_count, pull_request_count, lines_added, lines_removed,
        tool_invocations, error_count, last_updated
    )
    SELECT 
        COALESCE(NEW.session_id, ss.session_id),
        COALESCE(NEW.user_id, ss.user_id),
        COALESCE(NEW.user_email, ss.user_email),
        COALESCE(NEW.organization_id, ss.organization_id),
        COALESCE(s.start_time, ss.start_time),
        COALESCE(s.end_time, ss.end_time),
        CASE WHEN s.end_time IS NOT NULL THEN 
            CAST((julianday(s.end_time) - julianday(s.start_time)) * 86400 AS INTEGER)
        ELSE ss.duration_seconds END,
        COALESCE(ss.total_input_tokens, 0),
        COALESCE(ss.total_output_tokens, 0),
        COALESCE(ss.total_cache_creation_tokens, 0),
        COALESCE(ss.total_cache_read_tokens, 0),
        COALESCE(ss.total_cost_usd, 0.0),
        COALESCE(ss.commit_count, 0) + CASE WHEN NEW.event_type = 'commit' THEN 1 ELSE 0 END,
        COALESCE(ss.pull_request_count, 0) + CASE WHEN NEW.event_type = 'pull_request' THEN 1 ELSE 0 END,
        COALESCE(ss.lines_added, 0) + NEW.lines_added,
        COALESCE(ss.lines_removed, 0) + NEW.lines_removed,
        COALESCE(ss.tool_invocations, 0),
        COALESCE(ss.error_count, 0),
        NEW.created_at
    FROM sessions s
    LEFT JOIN session_summaries ss ON ss.session_id = NEW.session_id
    WHERE s.id = NEW.session_id;
END;