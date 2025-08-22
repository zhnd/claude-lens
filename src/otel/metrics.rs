use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Claude Code specific metric types based on Datadog monitoring patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClaudeCodeMetricType {
    // Core session metrics
    SessionCount,
    SessionDuration,
    
    // Token and cost tracking
    TokenUsage(TokenType),
    CostUsage,
    
    // Productivity metrics
    CommitCount,
    PullRequestCount,
    LinesOfCode(LinesType),
    
    // Tool usage tracking
    ToolUsage(String),
    
    // Error and performance tracking
    ErrorRate,
    ResponseTime,
    
    // Custom metrics
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenType {
    Input,
    Output,
    Total,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LinesType {
    Added,
    Removed,
    Modified,
    Total,
}

/// Enhanced metric structure with user context and classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedClaudeMetric {
    pub metric_type: ClaudeCodeMetricType,
    pub name: String,
    pub value: f64,
    pub timestamp: DateTime<Utc>,
    pub labels: HashMap<String, String>,
    
    // User identification (from blog post tags)
    pub user_id: Option<String>,
    pub user_email: Option<String>,
    pub organization_id: Option<String>,
    
    // Session context
    pub session_id: Option<String>,
    pub version: Option<String>,
    pub host: Option<String>,
    
    // Service context
    pub service: Option<String>,
}

/// Metric classifier to identify Claude Code metric types
pub struct MetricClassifier;

impl MetricClassifier {
    /// Classify a metric based on its name and labels
    pub fn classify_metric(name: &str, labels: &HashMap<String, String>) -> ClaudeCodeMetricType {
        match name {
            // Core Claude Code metrics from the blog
            "claude_code.session.count" => ClaudeCodeMetricType::SessionCount,
            "claude_code.token.usage" => {
                match labels.get("token_type").map(|s| s.as_str()) {
                    Some("input") => ClaudeCodeMetricType::TokenUsage(TokenType::Input),
                    Some("output") => ClaudeCodeMetricType::TokenUsage(TokenType::Output),
                    _ => ClaudeCodeMetricType::TokenUsage(TokenType::Total),
                }
            },
            "claude_code.cost.usage" => ClaudeCodeMetricType::CostUsage,
            "claude_code.commit.count" => ClaudeCodeMetricType::CommitCount,
            "claude_code.pull_request.count" => ClaudeCodeMetricType::PullRequestCount,
            "claude_code.lines_of_code.count" => {
                match labels.get("change_type").map(|s| s.as_str()) {
                    Some("added") => ClaudeCodeMetricType::LinesOfCode(LinesType::Added),
                    Some("removed") => ClaudeCodeMetricType::LinesOfCode(LinesType::Removed),
                    Some("modified") => ClaudeCodeMetricType::LinesOfCode(LinesType::Modified),
                    _ => ClaudeCodeMetricType::LinesOfCode(LinesType::Total),
                }
            },
            
            // Tool usage metrics
            name if name.starts_with("claude_code.tool.") => {
                let tool_name = name.strip_prefix("claude_code.tool.")
                    .unwrap_or("unknown")
                    .to_string();
                ClaudeCodeMetricType::ToolUsage(tool_name)
            },
            
            // Session duration
            "claude_code.session.duration" => ClaudeCodeMetricType::SessionDuration,
            
            // Error metrics
            "claude_code.error.rate" => ClaudeCodeMetricType::ErrorRate,
            
            // Performance metrics
            "claude_code.response.time" => ClaudeCodeMetricType::ResponseTime,
            
            // Fallback to custom metric
            _ => ClaudeCodeMetricType::Custom(name.to_string()),
        }
    }
    
    /// Extract user context from metric labels
    pub fn extract_user_context(labels: &HashMap<String, String>) -> UserContext {
        UserContext {
            user_id: labels.get("user.id").cloned(),
            user_email: labels.get("user.email").cloned(),
            organization_id: labels.get("organization.id").cloned(),
        }
    }
    
    /// Extract session context from metric labels
    pub fn extract_session_context(labels: &HashMap<String, String>) -> SessionContext {
        SessionContext {
            session_id: labels.get("session.id").cloned(),
            version: labels.get("version").cloned(),
            host: labels.get("host").cloned(),
            service: labels.get("service").cloned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserContext {
    pub user_id: Option<String>,
    pub user_email: Option<String>,
    pub organization_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SessionContext {
    pub session_id: Option<String>,
    pub version: Option<String>,
    pub host: Option<String>,
    pub service: Option<String>,
}

impl EnhancedClaudeMetric {
    /// Create an enhanced metric from basic metric data
    pub fn from_basic_metric(
        name: String,
        value: f64,
        timestamp: DateTime<Utc>,
        labels: HashMap<String, String>,
    ) -> Self {
        let metric_type = MetricClassifier::classify_metric(&name, &labels);
        let user_context = MetricClassifier::extract_user_context(&labels);
        let session_context = MetricClassifier::extract_session_context(&labels);
        
        Self {
            metric_type,
            name,
            value,
            timestamp,
            labels,
            user_id: user_context.user_id,
            user_email: user_context.user_email,
            organization_id: user_context.organization_id,
            session_id: session_context.session_id,
            version: session_context.version,
            host: session_context.host,
            service: session_context.service,
        }
    }
    
    /// Check if this metric represents a cost-related measurement
    pub fn is_cost_metric(&self) -> bool {
        matches!(self.metric_type, ClaudeCodeMetricType::CostUsage)
    }
    
    /// Check if this metric represents token usage
    pub fn is_token_metric(&self) -> bool {
        matches!(self.metric_type, ClaudeCodeMetricType::TokenUsage(_))
    }
    
    /// Check if this metric represents productivity data
    pub fn is_productivity_metric(&self) -> bool {
        matches!(
            self.metric_type,
            ClaudeCodeMetricType::CommitCount
                | ClaudeCodeMetricType::PullRequestCount
                | ClaudeCodeMetricType::LinesOfCode(_)
        )
    }
    
    /// Get metric category for grouping
    pub fn get_category(&self) -> MetricCategory {
        match &self.metric_type {
            ClaudeCodeMetricType::SessionCount | ClaudeCodeMetricType::SessionDuration => {
                MetricCategory::Session
            }
            ClaudeCodeMetricType::TokenUsage(_) => MetricCategory::Usage,
            ClaudeCodeMetricType::CostUsage => MetricCategory::Cost,
            ClaudeCodeMetricType::CommitCount
            | ClaudeCodeMetricType::PullRequestCount
            | ClaudeCodeMetricType::LinesOfCode(_) => MetricCategory::Productivity,
            ClaudeCodeMetricType::ToolUsage(_) => MetricCategory::Tools,
            ClaudeCodeMetricType::ErrorRate => MetricCategory::Errors,
            ClaudeCodeMetricType::ResponseTime => MetricCategory::Performance,
            ClaudeCodeMetricType::Custom(_) => MetricCategory::Custom,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricCategory {
    Session,
    Usage,
    Cost,
    Productivity,
    Tools,
    Errors,
    Performance,
    Custom,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_metric_classification() {
        let labels = HashMap::new();
        
        assert!(matches!(
            MetricClassifier::classify_metric("claude_code.session.count", &labels),
            ClaudeCodeMetricType::SessionCount
        ));
        
        assert!(matches!(
            MetricClassifier::classify_metric("claude_code.cost.usage", &labels),
            ClaudeCodeMetricType::CostUsage
        ));
        
        assert!(matches!(
            MetricClassifier::classify_metric("claude_code.tool.read", &labels),
            ClaudeCodeMetricType::ToolUsage(_)
        ));
    }
    
    #[test]
    fn test_token_type_classification() {
        let mut labels = HashMap::new();
        labels.insert("token_type".to_string(), "input".to_string());
        
        assert!(matches!(
            MetricClassifier::classify_metric("claude_code.token.usage", &labels),
            ClaudeCodeMetricType::TokenUsage(TokenType::Input)
        ));
    }
    
    #[test]
    fn test_user_context_extraction() {
        let mut labels = HashMap::new();
        labels.insert("user.id".to_string(), "user123".to_string());
        labels.insert("user.email".to_string(), "user@example.com".to_string());
        
        let context = MetricClassifier::extract_user_context(&labels);
        assert_eq!(context.user_id, Some("user123".to_string()));
        assert_eq!(context.user_email, Some("user@example.com".to_string()));
    }
}