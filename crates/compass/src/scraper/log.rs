//! Runtime logs
//!
//! Parse and record the logs emitted by the runtime to support
//! pos-processing and analysis.

use chrono::NaiveDateTime;
use duckdb;
use regex::Regex;
use tracing::{debug, trace};

use crate::error::Result;

#[derive(Debug, PartialEq, serde::Deserialize)]
enum LogLevel {
    #[serde(rename = "DEBUG_TO_FILE")]
    DebugToFile,
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

use std::sync::LazyLock;

impl LogRecord {
    fn parse(line: &str) -> Result<Self> {
        // Regex pattern: [timestamp] LEVEL - subject: message
        static RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"^\[([^\]]+)\]\s+(\w+)\s+-\s+([^:]+):\s+(.+)$").unwrap());

        let caps = RE.captures(line).ok_or_else(|| {
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

    fn record(&self, conn: &duckdb::Transaction, bookkeeper_id: usize) -> Result<()> {
        trace!("Recording log record: {:?}", self);
        conn.execute(
            "INSERT INTO logs (bookkeeper_lnk, timestamp, level, subject, message) VALUES (?, ?, ?, ?, ?)",
            duckdb::params![
                bookkeeper_id,
                self.timestamp.format("%Y-%m-%d %H:%M:%S").to_string(),
                format!("{:?}", self.level),
                &self.subject,
                &self.message,
            ],
        )?;
        Ok(())
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

#[derive(Debug)]
pub(super) struct RuntimeLogs(Vec<LogRecord>);

impl RuntimeLogs {
    fn parse(input: &str) -> Result<Self> {
        let records: Vec<LogRecord> = input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(LogRecord::parse)
            .collect::<Result<Vec<LogRecord>>>()?;
        Ok(RuntimeLogs(records))
    }

    pub(super) fn init_db(conn: &duckdb::Transaction) -> Result<()> {
        conn.execute_batch(
            r"
            CREATE SEQUENCE IF NOT EXISTS scrapper_log_seq START 1;
            CREATE TABLE IF NOT EXISTS logs (
              id INTEGER PRIMARY KEY DEFAULT NEXTVAL('scrapper_log_seq'),
              bookkeeper_lnk INTEGER REFERENCES bookkeeper(id) NOT NULL,
              timestamp TIMESTAMP,
              level VARCHAR,
              subject VARCHAR,
              message VARCHAR
            );
            ",
        )?;
        Ok(())
    }

    pub(super) async fn open<P: AsRef<std::path::Path>>(root: P) -> Result<Self> {
        let path = root.as_ref().join("logs").join("all.log");
        dbg!(&path);
        let content = tokio::fs::read_to_string(path).await?;
        let records = Self::parse(&content)?;
        Ok(records)
    }
}

#[cfg(test)]
/// Samples of runtime logs for testing purposes
pub(crate) mod sample {
    use crate::error::Result;
    use std::io::Write;

    pub(crate) fn as_text_v1() -> String {
        r#"
[2025-12-06 15:15:14,272] INFO - Task-1: Running COMPASS version 0.11.3.dev8+g69a75b7.d20251111
[2025-12-06 15:15:14,872] INFO - Task-1: Processing 250 jurisdiction(s)
[2025-12-06 15:15:14,272] INFO - Task-1: Running COMPASS
[2025-12-06 15:15:14,572] INFO - Jefferson County, Colorado: Running COMPASS
[2025-12-06 19:48:10,503] INFO - Task-1: Total runtime: 4:32:55
        "#
        .to_string()
    }

    pub(crate) fn as_file<P: AsRef<std::path::Path>>(path: P) -> Result<std::fs::File> {
        let mut file = std::fs::File::create(path).unwrap();
        dbg!(&file);
        writeln!(file, "{}", as_text_v1()).unwrap();
        Ok(file)
    }
}
