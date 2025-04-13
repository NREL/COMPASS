//! Parse and handle the scrapped ordinance information


#[allow(non_snake_case)]
#[derive(Debug, serde::Deserialize)]
pub(super) struct Ordinance {
    county: String,
    state: String,
    subdivison: Option<String>,
    jurisdiction_type: Option<String>,
    FIPS: u32,
    feature: String,
    value: Option<f64>,
    units: Option<String>,
    offset: Option<f64>,
    min_dist: Option<f64>,
    max_dist: Option<f64>,
    summary: Option<String>,
    ord_year: Option<u32>,
    section: Option<String>,
    source: Option<String>,
}
