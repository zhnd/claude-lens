use serde::{Deserialize, Serialize};
use std::{env, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub http_port: u16,
    pub otel_port: u16,
    pub database_path: String,
    pub cors_origins: Vec<String>,
    pub log_level: String,
    pub max_connections: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            http_port: 3000,
            otel_port: 4317,
            database_path: "./claude-scope.db".to_string(),
            cors_origins: vec![
                "http://localhost:3000".to_string(),
                "http://127.0.0.1:3000".to_string(),
            ],
            log_level: "info".to_string(),
            max_connections: 100,
        }
    }
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        let mut config = Self::default();

        if let Ok(port) = env::var("CLAUDE_SCOPE_HTTP_PORT") {
            if let Ok(port) = port.parse() {
                config.http_port = port;
            }
        }

        if let Ok(port) = env::var("CLAUDE_SCOPE_OTEL_PORT") {
            if let Ok(port) = port.parse() {
                config.otel_port = port;
            }
        }

        if let Ok(path) = env::var("CLAUDE_SCOPE_DATABASE_PATH") {
            config.database_path = path;
        }

        if let Ok(origins) = env::var("CLAUDE_SCOPE_CORS_ORIGINS") {
            config.cors_origins = origins
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
        }

        if let Ok(level) = env::var("CLAUDE_SCOPE_LOG_LEVEL") {
            config.log_level = level;
        }

        if let Ok(max_conn) = env::var("CLAUDE_SCOPE_MAX_CONNECTIONS") {
            if let Ok(max_conn) = max_conn.parse() {
                config.max_connections = max_conn;
            }
        }

        config
    }

    /// Load configuration from a TOML file
    pub fn from_file(path: &PathBuf) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| ConfigError::FileRead(e.to_string()))?;
        
        let config: Config = toml::from_str(&content)
            .map_err(|e| ConfigError::Parse(e.to_string()))?;
        
        Ok(config)
    }

    /// Save configuration to a TOML file
    pub fn save_to_file(&self, path: &PathBuf) -> Result<(), ConfigError> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| ConfigError::Serialize(e.to_string()))?;
        
        std::fs::write(path, content)
            .map_err(|e| ConfigError::FileWrite(e.to_string()))?;
        
        Ok(())
    }

    /// Validate configuration values
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.http_port == 0 {
            return Err(ConfigError::InvalidValue("HTTP port cannot be 0".to_string()));
        }

        if self.otel_port == 0 {
            return Err(ConfigError::InvalidValue("OpenTelemetry port cannot be 0".to_string()));
        }

        if self.http_port == self.otel_port {
            return Err(ConfigError::InvalidValue("HTTP and OpenTelemetry ports must be different".to_string()));
        }

        if self.database_path.is_empty() {
            return Err(ConfigError::InvalidValue("Database path cannot be empty".to_string()));
        }

        if self.max_connections == 0 {
            return Err(ConfigError::InvalidValue("Max connections cannot be 0".to_string()));
        }

        // Validate log level
        match self.log_level.to_lowercase().as_str() {
            "trace" | "debug" | "info" | "warn" | "error" => {},
            _ => return Err(ConfigError::InvalidValue(format!("Invalid log level: {}", self.log_level))),
        }

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    FileRead(String),
    #[error("Failed to write config file: {0}")]
    FileWrite(String),
    #[error("Failed to parse config: {0}")]
    Parse(String),
    #[error("Failed to serialize config: {0}")]
    Serialize(String),
    #[error("Invalid configuration value: {0}")]
    InvalidValue(String),
}