//! Parse and handle the Scrapper configuration information
//!
//! The setup used to run the scrapper is saved together with the output.
//! This module provides the support to work with that information, from
//! validating and parsing to loading it in the database.

use crate::error::Result;


#[allow(dead_code)]
#[derive(Debug)]
/// Configuration of the ordinance scrapper
pub(crate) struct ScrapperConfig {
    pub(crate) model: String,
    pub(crate) llm_service_rate_limit: u64,
    pub(crate) extra: String,
}

impl ScrapperConfig {
    #[allow(dead_code)]
    /// Extract the configuration from a JSON string
    pub(super) fn from_json(json: &str) -> Result<Self> {
        let mut v: serde_json::Map<String, serde_json::Value> = serde_json::from_str(json).unwrap();

        let model = v.remove("model").unwrap().as_str().unwrap().to_string();
        let llm_service_rate_limit = v
            .remove("llm_service_rate_limit")
            .unwrap()
            .as_u64()
            .unwrap();
        let extra = serde_json::to_string(&v).unwrap();

        Ok(Self {
            model,
            llm_service_rate_limit,
            extra,
        })
    }
}

mod test_scrapper_config {
    use super::ScrapperConfig;

    #[test]
    fn dev() {
        let data = r#"
        {
          "log_level": "DEBUG",
          "out_dir": ".",
          "county_fp": "counties.csv",
          "model": "gpt-4",
          "llm_call_kwargs":{
            "temperature": 0,
            "seed": 42,
            "timeout": 300
            },
          "llm_service_rate_limit": 50000,
          "td_kwargs": {
            "dir": "."
            },
          "tpe_kwargs": {
            "max_workers": 10
            },
          "ppe_kwargs": {
            "max_workers": 4
            }
        }"#;

        let config = ScrapperConfig::from_json(data).unwrap();

        assert_eq!(config.model, "gpt-4");
        assert_eq!(config.llm_service_rate_limit, 50000);
    }
}
