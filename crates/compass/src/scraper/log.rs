//! Runtime logs
//!
//! Parse and record the logs emitted by the runtime to support
//! pos-processing and analysis.

use chrono::NaiveDateTime;
use regex::Regex;

use crate::error::Result;

#[derive(Debug, PartialEq, serde::Deserialize)]
enum LogLevel {
    #[serde(rename = "TRACE")]
    Trace,
    #[serde(rename = "DEBUG")]
    Debug,
    #[serde(rename = "INFO")]
    Info,
    #[serde(rename = "WARNING")]
    Warning,
    #[serde(rename = "ERROR")]
    Error,
}

#[cfg(test)]
mod test_loglevel {
    use super::*;
    use serde_json;

    #[test]
    fn deserialize_trace() {
        let json = r#""TRACE""#;
        let level: LogLevel = serde_json::from_str(json).unwrap();
        assert!(matches!(level, LogLevel::Trace));
    }

    #[test]
    fn deserialize_debug() {
        let json = r#""DEBUG""#;
        let level: LogLevel = serde_json::from_str(json).unwrap();
        assert!(matches!(level, LogLevel::Debug));
    }

    #[test]
    fn deserialize_info() {
        let json = r#""INFO""#;
        let level: LogLevel = serde_json::from_str(json).unwrap();
        assert!(matches!(level, LogLevel::Info));
    }

    #[test]
    fn deserialize_warning() {
        let json = r#""WARNING""#;
        let level: LogLevel = serde_json::from_str(json).unwrap();
        assert!(matches!(level, LogLevel::Warning));
    }

    #[test]
    fn deserialize_error() {
        let json = r#""ERROR""#;
        let level: LogLevel = serde_json::from_str(json).unwrap();
        assert!(matches!(level, LogLevel::Error));
    }

    #[test]
    fn deserialize_invalid_variant() {
        let json = r#""INVALID""#;
        let result: std::result::Result<LogLevel, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn deserialize_lowercase_fails() {
        let json = r#""trace""#;
        let result: std::result::Result<LogLevel, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn deserialize_mixed_case_fails() {
        let json = r#""Trace""#;
        let result: std::result::Result<LogLevel, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn deserialize_in_struct() {
        #[derive(serde::Deserialize)]
        struct LogEntry {
            level: LogLevel,
            message: String,
        }

        let json = r#"{"level": "ERROR", "message": "Something went wrong"}"#;
        let entry: LogEntry = serde_json::from_str(json).unwrap();
        assert!(matches!(entry.level, LogLevel::Error));
        assert_eq!(entry.message, "Something went wrong");
    }

    #[test]
    fn deserialize_array_of_levels() {
        let json = r#"["TRACE", "INFO", "ERROR"]"#;
        let levels: Vec<LogLevel> = serde_json::from_str(json).unwrap();
        assert_eq!(levels.len(), 3);
        assert!(matches!(levels[0], LogLevel::Trace));
        assert!(matches!(levels[1], LogLevel::Info));
        assert!(matches!(levels[2], LogLevel::Error));
    }

    #[test]
    fn deserialize_with_whitespace() {
        let json = r#"  "INFO"  "#;
        let level: LogLevel = serde_json::from_str(json).unwrap();
        assert!(matches!(level, LogLevel::Info));
    }

    #[test]
    fn error_message_contains_valid_options() {
        let json = r#""FATAL""#;
        let result: std::result::Result<LogLevel, _> = serde_json::from_str(json);

        match result {
            Err(e) => {
                let error_msg = e.to_string();
                // The error message should mention valid variants
                assert!(error_msg.contains("unknown variant"));
            }
            Ok(_) => panic!("Expected deserialization to fail"),
        }
    }
}

#[derive(Debug)]
pub(super) struct LogRecord {
    timestamp: NaiveDateTime,
    level: LogLevel,
    subject: String,
    message: String,
}

impl LogRecord {
    fn parse(line: &str) -> Result<Self> {
        // Regex pattern: [timestamp] LEVEL - subject: message
        let re = Regex::new(r"^\[([^\]]+)\]\s+(\w+)\s+-\s+([^:]+):\s+(.+)$").unwrap();

        let caps = re.captures(line).ok_or_else(|| {
            crate::error::Error::Undefined(format!("Failed to parse log line: {}", line))
        })?;

        let timestamp_str = caps.get(1).unwrap().as_str().to_string();
        let timestamp = NaiveDateTime::parse_from_str(&timestamp_str, "%Y-%m-%d %H:%M:%S,%3f")
            .map_err(|e| {
                crate::error::Error::Undefined(format!(
                    "Failed to parse timestamp '{}': {}",
                    timestamp_str, e
                ))
            })?;

        let level_str = caps.get(2).unwrap().as_str();
        // Parse the log level
        let level = serde_json::from_str(&format!(r#""{}""#, level_str))
            .map_err(|e| format!("Invalid log level '{}': {}", level_str, e))
            .unwrap();

        let subject = caps.get(3).unwrap().as_str().to_string();
        let message = caps.get(4).unwrap().as_str().to_string();

        Ok(LogRecord {
            timestamp,
            level,
            subject,
            message,
        })
    }

    fn parse_lines(input: &str) -> Result<Vec<Self>> {
        input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(Self::parse)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_line() {
        let line = "[2025-12-06 15:15:14,272] INFO - Task-1: Running COMPASS";
        let record = LogRecord::parse(line).unwrap();

        assert_eq!(
            record.timestamp.date(),
            chrono::NaiveDate::from_ymd_opt(2025, 12, 6).unwrap()
        );
        assert_eq!(
            record.timestamp.time(),
            chrono::NaiveTime::from_hms_milli_opt(15, 15, 14, 272).unwrap()
        );

        assert!(matches!(record.level, LogLevel::Info));
        assert_eq!(record.subject, "Task-1");
        assert_eq!(record.message, "Running COMPASS");
    }
}

impl LogRecord {
    pub(super) fn init_db(conn: &duckdb::Transaction) -> Result<()> {
        conn.execute_batch(
            r"
            CREATE SEQUENCE IF NOT EXISTS scrapper_log_seq START 1;
            CREATE TABLE IF NOT EXISTS logs (
              id INTEGER PRIMARY KEY DEFAULT NEXTVAL('scrapper_log_seq'),
              timestamp TIMESTAMP,
              level VARCHAR,
              subject VARCHAR,
              message VARCHAR
            );
            ",
        )?;
        Ok(())
    }

    pub(super) async fn open<P: AsRef<std::path::Path>>(root: P) -> Result<Vec<Self>> {
        let path = root.as_ref().join("logs").join("all.log");
        dbg!(&path);
        let content = tokio::fs::read_to_string(path).await?;
        let records = Self::parse_lines(&content)?;
        Ok(records)
    }
}
