//! Scrapped document
//!

use tracing::trace;

use crate::error::Result;

/// Source document scrapped
pub(super) struct Source {
    name: String,
}

impl Source {
    pub(super) fn init_db(conn: &duckdb::Transaction) -> Result<()> {
        trace!("Initializing database for Source");

        conn.execute_batch(
            r"
            CREATE SEQUENCE document_sequence START 1;
            CREATE TABLE IF NOT EXISTS document (
              id INTEGER PRIMARY KEY DEFAULT NEXTVAL('document_sequence'),
              name TEXT NOT NULL,
              hash TEXT NOT NULL,
              origin TEXT,
              access_time TIMESTAMP,
              created_at TIMESTAMP NOT NULL DEFAULT NOW(),
              comments TEXT
              );",
        )?;

        conn.execute_batch(
            r"
            CREATE SEQUENCE source_sequence START 1;
            CREATE TABLE IF NOT EXISTS source (
              id INTEGER PRIMARY KEY DEFAULT NEXTVAL('source_sequence'),
              bookkeeper_lnk INTEGER REFERENCES bookkeeper(id) NOT NULL,
              document_lnk INTEGER REFERENCES source(id) NOT NULL,
              );",
        )?;

        Ok(())
    }
}
