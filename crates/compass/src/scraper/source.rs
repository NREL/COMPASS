//! Scrapped document
//!

use tracing::trace;

use crate::error::Result;

/// Source document scrapped
pub(super) struct Source {
    name: String,
    hash: String,
    origin: Option<String>,
    access_time: Option<String>,
}

impl Source {
    pub(super) fn init_db(conn: &duckdb::Transaction) -> Result<()> {
        trace!("Initializing database for Source");

        trace!("Creating table archive");
        // Store all individual documents scrapped
        conn.execute_batch(
            r"
            CREATE SEQUENCE IF NOT EXISTS archive_sequence START 1;
            CREATE TABLE IF NOT EXISTS archive (
              id INTEGER PRIMARY KEY DEFAULT NEXTVAL('archive_sequence'),
              name TEXT NOT NULL,
              hash TEXT NOT NULL,
              origin TEXT,
              access_time TIMESTAMP,
              created_at TIMESTAMP NOT NULL DEFAULT NOW(),
              comments TEXT
              );",
        )?;

        trace!("Creating table source");
        // Register the target documents of each scrapping run
        conn.execute_batch(
            r"
            CREATE SEQUENCE IF NOT EXISTS source_sequence START 1;
            CREATE TABLE IF NOT EXISTS source (
              id INTEGER PRIMARY KEY DEFAULT NEXTVAL('source_sequence'),
              bookkeeper_lnk INTEGER REFERENCES bookkeeper(id) NOT NULL,
              archive_lnk INTEGER REFERENCES source(id) NOT NULL,
              );",
        )?;

        Ok(())
    }

    pub(super) fn write(&self, conn: &duckdb::Transaction, commit_id: usize) -> Result<()> {
        // What about return the number of rows inserted?

        let origin = match &self.origin {
            Some(origin) => origin,
            None => {
                trace!("Missing origin for document {}", &self.name);
                "NULL"
            }
        };
        let access_time = match &self.access_time {
            Some(time) => time,
            None => {
                trace!("Missing access time for document {}", &self.name);
                "NULL"
            }
        };
        // Insert the source document into the database
        conn.execute(
            "INSERT INTO document (name, hash, origin, access_time) VALUES (?, ?, ?, ?)",
            &[&self.name, &self.hash, origin, access_time],
        )?;

        Ok(())
    }
}
