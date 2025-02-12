//! Parse and handle the Scrapper usage information

use crate::error::Result;

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
struct UsageValues {
    //event: String,
    request: u32,
    prompt_tokens: u32,
    response_tokens: u32,
}

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
pub(crate) struct Usage {
    pub(crate) total_time: f64,
    pub(crate) extra: String,
}

impl Usage {
    #[allow(dead_code)]
    pub(super) fn from_json(json: &str) -> Result<Self> {
        let mut v: serde_json::Map<String, serde_json::Value> = serde_json::from_str(json).unwrap();

        let total_time = v.remove("total_time_seconds").unwrap().as_f64().unwrap();
        let extra = serde_json::to_string(&v).unwrap();

        Ok(Self { total_time, extra })
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

        assert!((usage.total_time - 294.69257712364197).abs() <= f64::EPSILON);
    }
}
