//! Parse and handle the scrapped ordinance information

use tracing::trace;

use crate::error::Result;

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

impl Ordinance {
    pub(super) fn init_db(conn: &duckdb::Transaction) -> Result<()> {
        trace!("Initializing database for Ordinance");

        conn.execute_batch(
            r"
            CREATE SEQUENCE IF NOT EXISTS ordinance_sequence START 1;
            CREATE TABLE IF NOT EXISTS ordinance (
              id INTEGER PRIMARY KEY DEFAULT
                NEXTVAL('ordinance_sequence'),
              bookkeeper_lnk INTEGER REFERENCES bookkeeper(id) NOT NULL,
              county TEXT,
              state TEXT,
              subdivison TEXT,
              jurisdiction_type TEXT,
              FIPS INTEGER,
              feature TEXT,
              value REAL,
              units TEXT,
              offset REAL,
              min_dist REAL,
              max_dist REAL,
              summary TEXT,
              ord_year INTEGER,
              section TEXT,
              source TEXT
            );",
        )?;

        trace!("Database ready for Ordinance");
        Ok(())
    }
}

