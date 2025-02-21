//! Parse and handle the Scrapper usage information

use std::collections::HashMap;
use std::io::Read;

use crate::error::Result;

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
pub(super) struct Usage {
    pub(super) total_time_seconds: f64,
    pub(super) total_time: String,

    #[serde(flatten)]
    pub(super) jurisdiction: HashMap<String, UsagePerItem>,
}

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
pub(super) struct UsagePerItem {
    total_time_seconds: f64,
    total_time: String,

    #[serde(flatten)]
    events: HashMap<String, UsageValues>,
}

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
pub(super) struct UsageValues {
    //event: String,
    requests: u32,
    prompt_tokens: u32,
    response_tokens: u32,
}

impl Usage {
    pub(super) fn init_db(conn: &duckdb::Transaction) -> Result<()> {
        tracing::trace!("Initializing database for Usage");
        conn.execute_batch(
            r"
            CREATE SEQUENCE usage_sequence START 1;
            CREATE TABLE IF NOT EXISTS usage (
              id INTEGER PRIMARY KEY DEFAULT NEXTVAL('usage_sequence'),
              bookkeeper_lnk INTEGER REFERENCES bookkeeper(id) NOT NULL,
              total_time FLOAT NOT NULL,
              created_at TIMESTAMP NOT NULL DEFAULT NOW(),
              );

            CREATE SEQUENCE usage_per_item_sequence START 1;
            CREATE TABLE IF NOT EXISTS usage_per_item(
              id INTEGER PRIMARY KEY DEFAULT NEXTVAL('usage_per_item_sequence'),
              name TEXT NOT NULL,
              /* connection with file
              jurisdiction_lnk INTEGER REFERENCES jurisdiction(id) NOT NULL,
              */
              total_time FLOAT,
              total_requests INTEGER NOT NULL,
              total_prompt_tokens INTEGER NOT NULL,
              total_response_tokens INTEGER NOT NULL,
              );

            CREATE SEQUENCE usage_event_sequence START 1;
            CREATE TABLE IF NOT EXISTS usage_event (
              id INTEGER PRIMARY KEY DEFAULT NEXTVAL('usage_event_sequence'),
              usage_per_item_lnk INTEGER REFERENCES usage_per_item(id) NOT NULL,
              event TEXT NOT NULL,
              requests INTEGER NOT NULL,
              prompt_tokens INTEGER NOT NULL,
              response_tokens INTEGER NOT NULL,
              );",
        )?;

        Ok(())
    }

    pub(super) async fn open<P: AsRef<std::path::Path>>(root: P) -> Result<Self> {
        tracing::trace!("Opening Usage from {:?}", root.as_ref());

        let path = root.as_ref().join("usage.json");
        if !path.exists() {
            tracing::error!("Missing usage file: {:?}", path);
            return Err(crate::error::Error::Undefined(
                "Missing usage file".to_string(),
            ));
        }

        tracing::trace!("Identified Usage at {:?}", path);

        let file = std::fs::File::open(path);
        let mut reader = std::io::BufReader::new(file.unwrap());
        let mut buffer = String::new();
        let _ = reader.read_to_string(&mut buffer);

        let usage = Self::from_json(&buffer)?;
        tracing::trace!("Usage loaded: {:?}", usage);

        Ok(usage)
    }

    #[allow(dead_code)]
    pub(super) fn from_json(json: &str) -> Result<Self> {
        tracing::trace!("Parsing Usage as JSON");
        let usage: Usage = serde_json::from_str(json).unwrap();
        Ok(usage)
    }

    pub(super) fn write(&self, conn: &duckdb::Transaction, commit_id: usize) -> Result<()> {
        tracing::trace!("Writing Usage to the database {:?}", self);
        let usage_id: u32 = conn.query_row(
            "INSERT INTO usage (bookkeeper_lnk, total_time) VALUES (?, ?) RETURNING id",
            [&commit_id.to_string(), &self.total_time_seconds.to_string()],
            |row| row.get(0)
            ).expect("Failed to insert usage");
        tracing::trace!("Usage written to the database, id: {:?}", usage_id);

        for (jurisdiction_name, content) in &self.jurisdiction {
            tracing::trace!("Writing Usage-Item to the database: {:?}", jurisdiction_name);

            let item_id: u32 = conn.query_row(
                "INSERT INTO usage_per_item (name, total_time, total_requests, total_prompt_tokens, total_response_tokens) VALUES (?, ?, ?, ?, ?) RETURNING id",
                [jurisdiction_name, &content.total_time_seconds.to_string(), &content.events["tracker_totals"].requests.to_string(), &content.events["tracker_totals"].prompt_tokens.to_string(), &content.events["tracker_totals"].response_tokens.to_string()],
                |row| row.get(0)
                ).expect("Failed to insert usage_per_item");

            tracing::trace!("UsagePerItem written to the database, id: {:?}", item_id);

            for (event_name, event) in &content.events {
                tracing::trace!("Writing Usage-Event to the database: {:?}", event_name);

                conn.execute(
                    "INSERT INTO usage_event (usage_per_item_lnk, event, requests, prompt_tokens, response_tokens) VALUES (?, ?, ?, ?, ?)",
                    [&item_id.to_string(), event_name, &event.requests.to_string(), &event.prompt_tokens.to_string(), &event.response_tokens.to_string()]
                    ).expect("Failed to insert usage_event");
            }
        }


        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod sample {
    use crate::error::Result;
    use std::io::Write;

    pub(crate) fn as_text_v1() -> String {
        r#"
        {
          "total_time_seconds": 294.69257712364197,
          "total_time": "0:04:54.692577",
          "Decatur County, Indiana": {
            "document_location_validation": {
              "requests": 55,
              "prompt_tokens": 114614,
              "response_tokens": 1262
            },
            "document_content_validation": {
              "requests": 7,
              "prompt_tokens": 15191,
              "response_tokens": 477
            },
            "total_time_seconds": 294.64539074897766,
            "total_time": "0:04:54.645391",
            "tracker_totals": {
              "requests": 121,
              "prompt_tokens": 186099,
              "response_tokens": 6297
            }
          }
        }"#
        .to_string()
    }

    pub(crate) fn as_file<P: AsRef<std::path::Path>>(path: P) -> Result<std::fs::File> {
        let mut f = std::fs::File::create(path)?;
        write!(f, "{}", as_text_v1()).unwrap();
        Ok(f)
    }
}

#[cfg(test)]
mod test_scrapper_usage {
    use super::sample::as_text_v1;

    #[test]
    fn parse_json() {
        let usage = super::Usage::from_json(&as_text_v1()).unwrap();

        assert!((usage.total_time_seconds - 294.69257712364197).abs() <= f64::EPSILON);
    }
}
