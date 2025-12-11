//! Runtime logs
//!
//! Parse and record the logs emitted by the runtime to support
//! pos-processing and analysis.

#[derive(Debug, serde::Deserialize)]
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
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_deserialize_trace() {
        let json = r#""TRACE""#;
        let level: LogLevel = serde_json::from_str(json).unwrap();
        assert!(matches!(level, LogLevel::Trace));
    }

    #[test]
    fn test_deserialize_debug() {
        let json = r#""DEBUG""#;
        let level: LogLevel = serde_json::from_str(json).unwrap();
        assert!(matches!(level, LogLevel::Debug));
    }

    #[test]
    fn test_deserialize_info() {
        let json = r#""INFO""#;
        let level: LogLevel = serde_json::from_str(json).unwrap();
        assert!(matches!(level, LogLevel::Info));
    }

    #[test]
    fn test_deserialize_warning() {
        let json = r#""WARNING""#;
        let level: LogLevel = serde_json::from_str(json).unwrap();
        assert!(matches!(level, LogLevel::Warning));
    }

    #[test]
    fn test_deserialize_error() {
        let json = r#""ERROR""#;
        let level: LogLevel = serde_json::from_str(json).unwrap();
        assert!(matches!(level, LogLevel::Error));
    }

    #[test]
    fn test_deserialize_invalid_variant() {
        let json = r#""INVALID""#;
        let result: Result<LogLevel, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_lowercase_fails() {
        let json = r#""trace""#;
        let result: Result<LogLevel, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_mixed_case_fails() {
        let json = r#""Trace""#;
        let result: Result<LogLevel, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_in_struct() {
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
    fn test_deserialize_array_of_levels() {
        let json = r#"["TRACE", "INFO", "ERROR"]"#;
        let levels: Vec<LogLevel> = serde_json::from_str(json).unwrap();
        assert_eq!(levels.len(), 3);
        assert!(matches!(levels[0], LogLevel::Trace));
        assert!(matches!(levels[1], LogLevel::Info));
        assert!(matches!(levels[2], LogLevel::Error));
    }

    #[test]
    fn test_deserialize_with_whitespace() {
        let json = r#"  "INFO"  "#;
        let level: LogLevel = serde_json::from_str(json).unwrap();
        assert!(matches!(level, LogLevel::Info));
    }

    #[test]
    fn test_error_message_contains_valid_options() {
        let json = r#""FATAL""#;
        let result: Result<LogLevel, _> = serde_json::from_str(json);

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

struct LogRecord {
    timestamp: String,
    level: LogLevel,
    subject: String,
    message: String,
}
