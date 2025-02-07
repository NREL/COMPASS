#![deny(missing_docs)]

//! NREL's ordinance database

mod error;
mod scraper;

use duckdb::Connection;
use serde::Serialize;
use tracing::{debug, trace};

use error::Result;

/// Initialize the database
///
/// Create a new database as a local single file ready to store the ordinance
/// data.
pub fn init_db(path: &str) -> Result<()> {
    trace!("Creating a new database at {:?}", &path);

    let db = Connection::open(path)?;
    trace!("Database opened: {:?}", &db);

    db.execute_batch("SET VARIABLE ordinancedb_version = '0.0.1';")?;
    trace!("Defining ordinance data model version as: 0.0.1");

    db.execute_batch(
        "BEGIN;
    CREATE SEQUENCE bookkeeping_sequence START 1;
    CREATE TABLE bookkeeping (
        id INTEGER PRIMARY KEY DEFAULT NEXTVAL('bookkeeping_sequence'),
        hash TEXT NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT NOW(),
        username TEXT,
        comment TEXT,
        model TEXT
        );

    CREATE SEQUENCE scrapper_config_sequence START 1;
    CREATE TABLE scrapper_config (
      id INTEGER PRIMARY KEY DEFAULT NEXTVAL('scrapper_config_sequence'),
      model TEXT NOT NULL,
      llm_service_rate_limit INTEGER,
      extra TEXT,
      );

    CREATE SEQUENCE source_sequence START 1;
    CREATE TABLE source (
      id INTEGER PRIMARY KEY DEFAULT NEXTVAL('source_sequence'),
      bookkeeping_lnk INTEGER REFERENCES bookkeeping(id) NOT NULL,
      name TEXT NOT NULL,
      hash TEXT NOT NULL,
      origin TEXT,
      access_time TIMESTAMP,
      created_at TIMESTAMP NOT NULL DEFAULT NOW(),
      comments TEXT
      );

    INSTALL spatial;
    LOAD spatial;

    CREATE SEQUENCE jurisdiction_sequence START 1;
    CREATE TYPE jurisdiction_rank AS ENUM ('state', 'county', 'city', 'town', 'CCD', 'reservation', 'other');
    CREATE TABLE jurisdiction (
      id INTEGER PRIMARY KEY DEFAULT NEXTVAL('jurisdiction_sequence'),
      bookkeeping_lnk INTEGER REFERENCES bookkeeping(id) NOT NULL,
      name TEXT NOT NULL,
      FIPS INTEGER NOT NULL,
      geometry GEOMETRY NOT NULL,
      rank jurisdiction_rank NOT NULL,
      parent_id INTEGER REFERENCES jurisdiction(id),
      created_at TIMESTAMP NOT NULL DEFAULT NOW(),
      src TEXT,
      comments TEXT
      );

    CREATE SEQUENCE property_sequence START 1;
    CREATE TABLE property (
      id INTEGER PRIMARY KEY DEFAULT NEXTVAL('property_sequence'),

      county TEXT,
      state TEXT,
      FIPS INTEGER,
      feature TEXT NOT NULL,
      fixed_value TEXT,
      mult_value TEXT,
      mult_type TEXT,
      adder TEXT,
      min_dist TEXT,
      max_dist TEXT,
      value TEXT,
      units TEXT,
      ord_year TEXT,
      last_updated TEXT,
      section TEXT,
      source TEXT,

      comments TEXT
      );

    CREATE SEQUENCE usage_run_sequence START 1;
    CREATE TABLE usage_run (
      id INTEGER PRIMARY KEY DEFAULT NEXTVAL('usage_run_sequence'),
      bookkeeping_lnk INTEGER REFERENCES bookkeeping(id) NOT NULL,
      total_time FLOAT NOT NULL,
      extra TEXT,
      created_at TIMESTAMP NOT NULL DEFAULT NOW(),
      );

    CREATE SEQUENCE usage_per_item_sequence START 1;
    CREATE TABLE usage_per_item(
      id INTEGER PRIMARY KEY DEFAULT NEXTVAL('usage_per_item_sequence'),
      /* connection with file */
      jurisdiction_lnk INTEGER REFERENCES jurisdiction(id) NOT NULL,
      total_time FLOAT,
      total_requests INTEGER NOT NULL,
      total_prompt_tokens INTEGER NOT NULL,
      total_response_tokens INTEGER NOT NULL,
      );

    CREATE SEQUENCE usage_event_sequence START 1;
    CREATE TABLE usage_event (
      id INTEGER PRIMARY KEY DEFAULT NEXTVAL('usage_event_sequence'),
      usage_per_item_lnk INTEGER REFERENCES usage_per_item(id) NOT NULL,
      event TEXT NOT NULL,
      requests INTEGER NOT NULL,
      prompt_tokens INTEGER NOT NULL,
      response_tokens INTEGER NOT NULL,
      );

    COMMIT;",
    )?;

    trace!("Database initialized");

    println!("{}", db.is_autocommit());
    Ok(())
}

#[allow(dead_code)]
/// Scan and load features from a CSV file
///
/// Proof of concept. Parse a CSV file and load the features into the
/// database.
pub fn scan_features<P: AsRef<std::path::Path> + std::fmt::Debug>(
    mut database: duckdb::Connection,
    username: &String,
    ordinance_path: P,
) {

    // insert into bookkeeping (hash, username) and get the pk to be used in all the following
    // inserts.
    let conn = database.transaction().unwrap();
    trace!("Transaction started");
    // conn.execute("BEGIN TRANSACTION", []).unwrap();
    let commit_id = conn.execute("INSERT INTO bookkeeping (hash, username) VALUES (?, ?) RETURNING id", [
        "dummy hash".to_string(),
        username.to_string(),
    ]);
    debug!("Commit id: {:?}", commit_id);

    dbg!(&ordinance_path);
    let raw_filename = ordinance_path.as_ref().join("ord_db.csv");
    dbg!(&raw_filename);
    dbg!("========");

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let ordinance = rt
        .block_on(scraper::ScrappedOrdinance::open(ordinance_path))
        .unwrap();
    dbg!(&ordinance);
    /*
    let scrapper_config = rt.block_on(ordinance.config()).unwrap();
    let scrapper_usage = rt.block_on(ordinance.usage()).unwrap();
    dbg!(&scrapper_usage);

    let conn = database;

    let mut stmt = conn
        .prepare_cached(
            "INSERT INTO usage_run (bookkeeping_lnk, total_time, extra) VALUES (?, ?, ?)",
        )
        .unwrap();
    stmt.execute([
        "1".to_string(),
        scrapper_usage.total_time.to_string(),
        scrapper_usage.extra,
    ])
    .unwrap();

    let mut stmt = conn
        .prepare_cached(
            "INSERT INTO scrapper_config (model, llm_service_rate_limit, extra) VALUES (?, ?, ?)",
        )
        .unwrap();
    stmt.execute([
        scrapper_config.model,
        scrapper_config.llm_service_rate_limit.to_string(),
        scrapper_config.extra,
    ])
    .unwrap();

    let mut rdr = csv::Reader::from_path(raw_filename).unwrap();
    let mut stmt = conn.prepare_cached("INSERT INTO property (county, state, FIPS, feature, fixed_value, mult_value, mult_type, adder, min_dist, max_dist, value, units, ord_year, last_updated, section, source, comments) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)").unwrap();
    for result in rdr.records() {
        let record = result.unwrap();
        // println!("{:?}", record);
        stmt.execute([
            record[0].to_string(),
            record[1].to_string(),
            record[2].to_string(),
            record[3].to_string(),
            record[4].to_string(),
            record[5].to_string(),
            record[6].to_string(),
            record[7].to_string(),
            record[8].to_string(),
            record[9].to_string(),
            record[10].to_string(),
            record[11].to_string(),
            record[12].to_string(),
            record[13].to_string(),
            record[14].to_string(),
            record[15].to_string(),
            record[16].to_string(),
        ])
        .unwrap();
    }

    */
    //let df = polars::io::csv::read::CsvReadOptions::default().with_has_header(true).try_into_reader_with_file_path(Some("sample.csv".into())).unwrap().finish();

    conn.commit().unwrap();
    debug!("Transaction committed");

}

#[derive(Debug, Serialize)]
struct Ordinance {
    county: String,
    state: String,
    fips: i32,
    feature: String,
}

/// Export the database
///
/// Currently, it is a proof of concept. It reads the database and prints
/// some fields to the standard output in CSV format.
pub fn export_db(db_filename: &str) {
    let conn = Connection::open(db_filename).unwrap();
    let mut stmt = conn
        .prepare("SELECT county, state, fips, feature FROM property")
        .expect("Failed to prepare statement");
    //dbg!("Row count", stmt.row_count());
    let row_iter = stmt
        .query_map([], |row| {
            Ok(Ordinance {
                county: row.get(0)?,
                state: row.get(1)?,
                fips: row.get(2)?,
                feature: row.get(3)?,
            })
        })
        .expect("Failed to query");

    let mut wtr = csv::Writer::from_writer(std::io::stdout());

    for row in row_iter {
        wtr.serialize(row.unwrap()).unwrap();
    }
    wtr.flush().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let _ = init_db("test");
    }
}
