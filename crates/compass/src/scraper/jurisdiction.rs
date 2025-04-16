//! Parse and handle the scrapped jurisdiction information


use std::collections::HashMap;

use serde::Deserialize;
use tracing::{trace, warn, error};

use crate::Result;

// An arbitrary limit to protect against maliciously large JSON files
const MAX_JSON_FILE_SIZE: u64 = 5 * 1024 * 1024; // 10 MB

#[derive(Deserialize, Debug)]
pub(super) struct Collection {
    pub(super) jurisdictions: Vec<Jurisdiction>,
}

#[derive(Deserialize, Debug)]
pub(super) struct Jurisdiction {
    full_name: String,
    county: String,
    state: String,
    subdivision: Option<String>,
    jurisdiction_type: Option<String>,
    FIPS: u32,
    found: bool,
    total_time: f64,
    total_time_string: String,
    documents: Option<Vec<Document>>,
}

#[derive(Deserialize, Debug)]
pub(super) struct Document {
    source: String,
    ord_year: u16,
    num_pages: u16,
    checksum: String,
}

impl Collection {
    fn from_json(content: &str) -> Result<Self> {
        warn!("Parsing jurisdictions from JSON: {:?}", content);
        let collection: Collection = serde_json::from_str(content).unwrap();
        Ok(collection)
    }

    pub(super) async fn open<P: AsRef<std::path::Path>>(root: P) -> Result<Self> {
        warn!("Opening jurisdictions collection");

        let path = root.as_ref().join("jurisdictions.json");
        if !path.exists() {
            error!("Missing jurisdictions.json file");
            return Err(crate::error::Error::Undefined(
                "Missing jurisdictions.json file".to_string(),
            ));
        }

        warn!("Identified jurisdictions.json file");

        let file_size = tokio::fs::metadata(&path).await?.len();
        if file_size > MAX_JSON_FILE_SIZE {
            error!("Jurisdictions file too large: {:?}", file_size);
            return Err(crate::error::Error::Undefined(
                "jurisdictions.json file is too large".to_string(),
            ));
        }

        let content = tokio::fs::read_to_string(path).await?;
        let jurisdictions = Self::from_json(&content)?;
        warn!("Jurisdictions loaded: {:?}", jurisdictions);

        Ok(jurisdictions)
    }
}
