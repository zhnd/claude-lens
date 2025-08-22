pub mod receiver;
pub mod metrics;

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// Claude Code specific metric names that we expect to receive
pub const CLAUDE_CODE_METRICS: &[&str] = &[
    "claude_code.token.usage",
    "claude_code.cost.usage", 
    "claude_code.session.count",
    "claude_code.lines_of_code.count",
    "claude_code.commit.count",
    "claude_code.pull_request.count",
];

// Claude Code specific event types
pub const CLAUDE_CODE_EVENTS: &[&str] = &[
    "user_prompt_submitted",
    "tool_result",
    "api_request",
    "api_request_failed", 
    "tool_permission_decision",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedMetric {
    pub name: String,
    pub value: f64,
    pub timestamp: DateTime<Utc>,
    pub labels: HashMap<String, String>,
    pub session_id: Option<String>,
    pub metric_type: MetricType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricType {
    TokenUsage { token_type: TokenType },
    CostUsage { model: String },
    SessionCount,
    LinesOfCode { change_type: CodeChangeType },
    CommitCount,
    PullRequestCount,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenType {
    Input,
    Output,
    CacheCreation,
    CacheRead,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CodeChangeType {
    Added,
    Removed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedEvent {
    pub event_type: EventType,
    pub timestamp: DateTime<Utc>,
    pub attributes: HashMap<String, String>,
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    UserPromptSubmitted,
    ToolResult { tool_name: String },
    ApiRequest { endpoint: String },
    ApiRequestFailed { error_code: String },
    ToolPermissionDecision { tool_name: String, allowed: bool },
    Other { name: String },
}

// Validation and processing functions
pub fn validate_claude_code_metric(name: &str) -> bool {
    CLAUDE_CODE_METRICS.contains(&name) || name.starts_with("claude_code.")
}

pub fn validate_claude_code_event(event_name: &str) -> bool {
    CLAUDE_CODE_EVENTS.contains(&event_name)
}

pub fn classify_metric(name: &str, labels: &HashMap<String, String>) -> MetricType {
    match name {
        "claude_code.token.usage" => {
            let token_type = match labels.get("type").map(|s| s.as_str()) {
                Some("input") => TokenType::Input,
                Some("output") => TokenType::Output,
                Some("cache_creation") => TokenType::CacheCreation,
                Some("cache_read") => TokenType::CacheRead,
                _ => TokenType::Input, // Default
            };
            MetricType::TokenUsage { token_type }
        }
        "claude_code.cost.usage" => {
            let model = labels.get("model")
                .unwrap_or(&"unknown".to_string())
                .clone();
            MetricType::CostUsage { model }
        }
        "claude_code.session.count" => MetricType::SessionCount,
        "claude_code.lines_of_code.count" => {
            let change_type = match labels.get("type").map(|s| s.as_str()) {
                Some("added") => CodeChangeType::Added,
                Some("removed") => CodeChangeType::Removed,
                _ => CodeChangeType::Added, // Default
            };
            MetricType::LinesOfCode { change_type }
        }
        "claude_code.commit.count" => MetricType::CommitCount,
        "claude_code.pull_request.count" => MetricType::PullRequestCount,
        _ => MetricType::Other,
    }
}

pub fn classify_event(name: &str, attributes: &HashMap<String, String>) -> EventType {
    match name {
        "user_prompt_submitted" => EventType::UserPromptSubmitted,
        "tool_result" => {
            let tool_name = attributes.get("tool_name")
                .unwrap_or(&"unknown".to_string())
                .clone();
            EventType::ToolResult { tool_name }
        }
        "api_request" => {
            let endpoint = attributes.get("endpoint")
                .unwrap_or(&"unknown".to_string())
                .clone();
            EventType::ApiRequest { endpoint }
        }
        "api_request_failed" => {
            let error_code = attributes.get("error_code")
                .unwrap_or(&"unknown".to_string())
                .clone();
            EventType::ApiRequestFailed { error_code }
        }
        "tool_permission_decision" => {
            let tool_name = attributes.get("tool_name")
                .unwrap_or(&"unknown".to_string())
                .clone();
            let allowed = attributes.get("allowed")
                .and_then(|s| s.parse::<bool>().ok())
                .unwrap_or(false);
            EventType::ToolPermissionDecision { tool_name, allowed }
        }
        _ => EventType::Other { name: name.to_string() },
    }
}

// Session summary computation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSummary {
    pub session_id: String,
    pub total_tokens_input: u64,
    pub total_tokens_output: u64,
    pub total_tokens_cache_creation: u64,
    pub total_tokens_cache_read: u64,
    pub total_cost: f64,
    pub total_commits: u64,
    pub total_pull_requests: u64,
    pub lines_added: u64,
    pub lines_removed: u64,
    pub tool_usage: HashMap<String, u64>,
    pub api_requests: u64,
    pub api_failures: u64,
    pub last_updated: DateTime<Utc>,
}

impl Default for SessionSummary {
    fn default() -> Self {
        Self {
            session_id: String::new(),
            total_tokens_input: 0,
            total_tokens_output: 0,
            total_tokens_cache_creation: 0,
            total_tokens_cache_read: 0,
            total_cost: 0.0,
            total_commits: 0,
            total_pull_requests: 0,
            lines_added: 0,
            lines_removed: 0,
            tool_usage: HashMap::new(),
            api_requests: 0,
            api_failures: 0,
            last_updated: Utc::now(),
        }
    }
}

impl SessionSummary {
    pub fn update_from_metric(&mut self, metric: &ProcessedMetric) {
        match &metric.metric_type {
            MetricType::TokenUsage { token_type } => {
                match token_type {
                    TokenType::Input => self.total_tokens_input += metric.value as u64,
                    TokenType::Output => self.total_tokens_output += metric.value as u64,
                    TokenType::CacheCreation => self.total_tokens_cache_creation += metric.value as u64,
                    TokenType::CacheRead => self.total_tokens_cache_read += metric.value as u64,
                }
            }
            MetricType::CostUsage { .. } => {
                self.total_cost += metric.value;
            }
            MetricType::LinesOfCode { change_type } => {
                match change_type {
                    CodeChangeType::Added => self.lines_added += metric.value as u64,
                    CodeChangeType::Removed => self.lines_removed += metric.value as u64,
                }
            }
            MetricType::CommitCount => {
                self.total_commits += metric.value as u64;
            }
            MetricType::PullRequestCount => {
                self.total_pull_requests += metric.value as u64;
            }
            _ => {} // Ignore other metrics for summary
        }
        self.last_updated = Utc::now();
    }
    
    pub fn update_from_event(&mut self, event: &ProcessedEvent) {
        match &event.event_type {
            EventType::ToolResult { tool_name } => {
                *self.tool_usage.entry(tool_name.clone()).or_insert(0) += 1;
            }
            EventType::ApiRequest { .. } => {
                self.api_requests += 1;
            }
            EventType::ApiRequestFailed { .. } => {
                self.api_failures += 1;
            }
            _ => {} // Ignore other events for summary
        }
        self.last_updated = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_claude_code_metric() {
        assert!(validate_claude_code_metric("claude_code.token.usage"));
        assert!(validate_claude_code_metric("claude_code.cost.usage"));
        assert!(!validate_claude_code_metric("other.metric"));
    }
    
    #[test]
    fn test_classify_metric() {
        let mut labels = HashMap::new();
        labels.insert("type".to_string(), "input".to_string());
        
        match classify_metric("claude_code.token.usage", &labels) {
            MetricType::TokenUsage { token_type: TokenType::Input } => {},
            _ => panic!("Expected TokenUsage with Input type"),
        }
    }
    
    #[test] 
    fn test_session_summary_update() {
        let mut summary = SessionSummary::default();
        
        let metric = ProcessedMetric {
            name: "claude_code.token.usage".to_string(),
            value: 100.0,
            timestamp: Utc::now(),
            labels: HashMap::from([("type".to_string(), "input".to_string())]),
            session_id: Some("test-session".to_string()),
            metric_type: MetricType::TokenUsage { token_type: TokenType::Input },
        };
        
        summary.update_from_metric(&metric);
        assert_eq!(summary.total_tokens_input, 100);
    }
}