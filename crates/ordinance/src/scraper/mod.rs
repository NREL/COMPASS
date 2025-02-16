//! Support for the ordinance scrapper output

mod config;
mod usage;

use std::path::{Path, PathBuf};

use tracing::{self, trace};

use crate::error;
use crate::error::Result;
use config::ScrapperConfig;
use usage::ScrapperUsage;

pub(crate) const SCRAPPED_ORDINANCE_VERSION: &str = "0.0.1";

// Concepts
// - Lazy loading a scrapper output
//   - Early validation. Not necessary complete, but able to abort early
//     if identifies any major problem.
//   - Handle multiple versions. Identify right the way if the output is
//     a compatible version, and how to handle it.
//     - Define the trait and implement that on multiple readers for different
//       versions.
// - Async loading into DB
// - Logging operations

// Some concepts:
//
// - One single ordinance output is loaded and abstracted as a
//   ScrappedOrdinance. Everything inside should be accessible through this
//   abstraction.
// - It is possible to operate in multiple ordinance outputs at once, such
//   as loading multiple ordinance outputs into the database.
// - The ScrappedOrdinance should implement a hash estimate, which will
//   be used to identify the commit in the database.
// - Open ScrappedOrdinance is an async operation, and accessing/parsing
//   each component is also async. Thus, it can load into DB as it goes
//   until complete all components.

#[allow(dead_code)]
#[derive(Debug)]
/// Abstraction for the ordinance scrapper raw output
///
/// The ordinance scrapper outputs a standard directory with multiple files
/// and sub-directories. This struct abstracts the access to such output.
pub(crate) struct ScrappedOrdinance {
    format_version: String,
    root: PathBuf,
    config: Option<ScrapperConfig>,
    usage: Option<ScrapperUsage>,
}

impl ScrappedOrdinance {
    pub(super) fn init_db(conn: &duckdb::Transaction) -> Result<()> {
        tracing::trace!("Initializing ScrappedOrdinance database");
        config::ScrapperConfig::init_db(conn)?;
        usage::ScrapperUsage::init_db(conn)?;

        Ok(())
    }

    // Keep in mind a lazy state.
    #[allow(dead_code)]
    /// Open an existing scrapped ordinance folder
    pub(crate) async fn open<P: AsRef<Path>>(root: P) -> Result<Self> {
        trace!("Opening scrapped ordinance");

        let root = root.as_ref().to_path_buf();
        trace!("Scrapper output located at: {:?}", root);

        // Do some validation before returning a ScrappedOrdinance

        if !root.exists() {
            trace!("Root path does not exist");
            return Err(error::Error::Undefined("Path does not exist".to_string()));
        }

        /*
        let features_file = root.join("ord_db.csv");
        if !features_file.exists() {
            trace!("Missing features file: {:?}", features_file);
            return Err(error::Error::Undefined(
                "Features file does not exist".to_string(),
            ));
        }
        */
        let config = config::ScrapperConfig::open(&root).await?;
        let usage = usage::ScrapperUsage::open(&root).await?;

        /*
        let usage_file = root.join("ord_db.csv");
        if !usage_file.exists() {
            trace!("Missing usage file: {:?}", usage_file);
            return Err(error::Error::Undefined(
                "Features file does not exist".to_string(),
            ));
        }
        */

        Ok(Self {
            root,
            format_version: SCRAPPED_ORDINANCE_VERSION.to_string(),
            config: Some(config),
            usage: Some(usage),
        })
    }

    #[allow(dead_code)]
    pub(crate) async fn push(&self, conn: &mut duckdb::Connection) -> Result<()> {
        tracing::trace!("Pushing scrapped ordinance into the database");
        let conn = conn.transaction().unwrap();
        tracing::trace!("Transaction started");

        self.config().await.unwrap().write(&conn).unwrap();
        self.usage().await.unwrap().write(&conn).unwrap();

        tracing::trace!("Committing transaction");
        conn.commit()?;

        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) async fn config(&self) -> Result<ScrapperConfig> {
        let config_file = &self.root.join("config.json");
        if !config_file.exists() {
            trace!("Missing config file: {:?}", config_file);
            return Err(error::Error::Undefined(
                "Features file does not exist".to_string(),
            ));
        }

        let config = ScrapperConfig::from_json(&std::fs::read_to_string(config_file)?)
            .expect("Failed to parse config file");

        Ok(config)
    }

    #[allow(dead_code)]
    pub(crate) async fn usage(&self) -> Result<ScrapperUsage> {
        let usage_file = &self.root.join("usage.json");
        if !usage_file.exists() {
            trace!("Missing usage file: {:?}", usage_file);
            return Err(error::Error::Undefined(
                "Features file does not exist".to_string(),
            ));
        }

        let usage = ScrapperUsage::from_json(&std::fs::read_to_string(usage_file)?)
            .expect("Failed to parse usage file");

        Ok(usage)
    }
}

#[cfg(test)]
mod tests {
    use super::ScrappedOrdinance;
    use std::io::Write;

    #[tokio::test]
    /// Opening an inexistent path should give an error
    async fn open_inexistent_path() {
        let tmp = tempfile::tempdir().unwrap();
        let target = tmp.path().join("inexistent");

        // First confirm that the path does not exist
        assert!(!target.exists());
        ScrappedOrdinance::open(target).await.unwrap_err();
    }

    #[tokio::test]
    /// Open a Scrapped Ordinance raw output
    async fn open_scrapped_ordinance() {
        // A sample ordinance file for now.
        let target = tempfile::tempdir().unwrap();
        let ordinance_filename = target.path().join("ord_db.csv");
        let mut ordinance_file = std::fs::File::create(ordinance_filename).unwrap();
        writeln!(ordinance_file, "county,state,FIPS,feature,fixed_value,mult_value,mult_type,adder,min_dist,max_dist,value,units,ord_year,last_updated,section,source,comment").unwrap();
        writeln!(ordinance_file, "county-1,state-1,FIPS-1,feature-1,fixed_value-1,mult_value-1,mult_type-1,adder-1,min_dist-1,max_dist-1,value-1,units-1,ord_year-1,last_updated-1,section-1,source-1,comment").unwrap();

        ScrappedOrdinance::open(target).await.unwrap();
    }
}
