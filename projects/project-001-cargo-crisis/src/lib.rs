// ISSUE: This file contains LIBRARY code but it's in main.rs
// Library code should be in lib.rs so other crates can import it.
// The binary (CLI) code should be separate.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A structured log entry used by the logforge system.
///
/// This is the core type that all logging operations work with.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub message: String,
    pub source: String,
}

/// Log severity levels supported by logforge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

impl LogEntry {
    /// Create a new log entry with the current timestamp.
    pub fn new(level: LogLevel, message: impl Into<String>, source: impl Into<String>) -> Self {
        Self {
            timestamp: Utc::now(),
            level,
            message: message.into(),
            source: source.into(),
        }
    }

    /// Serialize the log entry to JSON format.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Parse a log entry from JSON format.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

impl LogLevel {
    /// Returns true if this level is considered severe (Error or Critical).
    pub fn is_severe(&self) -> bool {
        matches!(self, LogLevel::Error | LogLevel::Critical)
    }

    /// Returns the numeric priority (higher = more severe).
    pub fn priority(&self) -> u8 {
        match self {
            LogLevel::Debug => 0,
            LogLevel::Info => 1,
            LogLevel::Warning => 2,
            LogLevel::Error => 3,
            LogLevel::Critical => 4,
        }
    }
}

/// Validate that a log configuration is well-formed.
/// Used by the logforge-validate binary.
pub fn validate_config(config_json: &str) -> Result<(), String> {
    // Simple validation: just check it's valid JSON with expected fields
    let value: serde_json::Value = serde_json::from_str(config_json)
        .map_err(|e| format!("Invalid JSON: {}", e))?;

    if !value.is_object() {
        return Err("Config must be a JSON object".to_string());
    }

    let obj = value.as_object().unwrap();

    if !obj.contains_key("log_level") {
        return Err("Missing required field: log_level".to_string());
    }

    if !obj.contains_key("output") {
        return Err("Missing required field: output".to_string());
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_severity() {
        assert!(!LogLevel::Debug.is_severe());
        assert!(!LogLevel::Info.is_severe());
        assert!(!LogLevel::Warning.is_severe());
        assert!(LogLevel::Error.is_severe());
        assert!(LogLevel::Critical.is_severe());
    }

    #[test]
    fn test_log_level_priority() {
        assert!(LogLevel::Critical.priority() > LogLevel::Error.priority());
        assert!(LogLevel::Error.priority() > LogLevel::Warning.priority());
        assert!(LogLevel::Warning.priority() > LogLevel::Info.priority());
        assert!(LogLevel::Info.priority() > LogLevel::Debug.priority());
    }

    #[test]
    fn test_log_entry_json_roundtrip() {
        let entry = LogEntry::new(LogLevel::Info, "Test message", "test_source");
        let json = entry.to_json().expect("Failed to serialize");
        let parsed = LogEntry::from_json(&json).expect("Failed to parse");

        assert_eq!(entry.level, parsed.level);
        assert_eq!(entry.message, parsed.message);
        assert_eq!(entry.source, parsed.source);
    }

    #[test]
    fn test_validate_config_valid() {
        let config = r#"{"log_level": "info", "output": "stdout"}"#;
        assert!(validate_config(config).is_ok());
    }

    #[test]
    fn test_validate_config_missing_fields() {
        let config = r#"{"log_level": "info"}"#;
        assert!(validate_config(config).is_err());
    }
}
