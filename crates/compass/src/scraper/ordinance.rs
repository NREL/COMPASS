//! Parse and handle the scrapped ordinance information

use tracing::trace;

use crate::error::Result;

#[derive(Debug)]
pub(super) struct Ordinance (Vec<OrdinanceRecord>);

#[allow(non_snake_case)]
#[derive(Debug, serde::Deserialize)]
pub(super) struct OrdinanceRecord {
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

    /// Open the quantiative ordinance from scrapped output
    pub(super) async fn open<P: AsRef<std::path::Path>>(root: P) -> Result<Ordinance> {
        trace!("Opening quantitative ordinance of {:?}", root.as_ref());

        let path = root.as_ref().join("quantitative_ordinances.csv");
        if !path.exists() {
            trace!("Missing quantitative ordinance file: {:?}", path);
            return Err(crate::error::Error::Undefined(
                "Missing quantitative ordinance file".to_string(),
            ));
        }

        trace!("Identified quantitative ordinance at {:?}", path);

        /*
        let df = CsvReadOptions::default()
            .with_has_header(true)
            .try_into_reader_with_file_path(path.into()).unwrap()
            .finish()
            .unwrap();
        */

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b',')
            .from_path(&path).unwrap();

        trace!("Quantitative reader {:?}", rdr);

        let mut output = Vec::new();
        for result in rdr.deserialize() {
            let record: OrdinanceRecord = match result {
                Ok(record) => record,
                Err(_) => {
                    trace!("Error {:?}", result);
                    continue;
                }
            };
            output.push(record);

        }
        trace!("Quantitative ordinance records {:?}", output);

        Ok(output)
    }
}
