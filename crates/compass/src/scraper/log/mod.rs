//! Runtime logs
//!
//! Parse and record the logs emitted by the runtime to support
//! pos-processing and analysis.

mod loglevel;

use std::sync::LazyLock;

use chrono::NaiveDateTime;
use regex::Regex;
use tracing::{debug, trace};

use crate::error::Result;
use loglevel::LogLevel;

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
    /// Parse runtime logs from text input
    ///
    /// # Notes
    /// - For now, hardcoded to only keep INFO, WARNING, and ERROR levels.
    ///   These logs are quite long with the more verbose levels. Since I
    ///   collect everything, it is better to filter and reduce it here.
    ///   In the future, consider returning an iterator instead.
    /// - Ignore mulltiple lines messages.
    /// - Lines that fail to parse are skipped with a warning.
    fn parse(input: &str) -> Result<Self> {
        let records: Vec<LogRecord> = input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .filter_map(|line| match LogRecord::parse(line) {
                Ok(record) => {
                    trace!("Parsed log line: {}", line);
                    Some(record)
                }
                Err(e) => {
                    trace!("Failed to parse log line: {}. Error: {}", line, e);
                    None
                }
            })
            .filter(|record| {
                (record.level == LogLevel::Info)
                    || (record.level == LogLevel::Warning)
                    || (record.level == LogLevel::Error)
            })
            .map(|record| {
                debug!("Keeping log record: {:?}", record);
                record
            })
            .collect();
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

    pub(super) fn record(&self, conn: &duckdb::Transaction, commit_id: usize) -> Result<()> {
        // debug!("Recording log: {:?}", self);
        debug!("Recording runtime logs");

        for record in &self.0 {
            record.record(conn, commit_id)?;
        }

        Ok(())
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
