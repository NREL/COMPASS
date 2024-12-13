//! Support for the ordinance scrapper output

use std::path::{Path, PathBuf};

use crate::error;
use crate::error::Result;

// Concepts
// - Lazy loading a scrapper output
//   - Early validation. Not necessary complete, but able to abort early
//     if identifies a problem.
//   - Handle multiple versions. Identify right the way if the output is
//     a compatible version, and how to handle it.
// - Async loading into DB
// - Logging operations

#[allow(dead_code)]
#[derive(Debug)]
/// Abstraction for the ordinance scrapper raw output
///
/// The ordinance scrapper outputs a standard directory with multiple files
/// and sub-directories. This struct abstracts the access to such output.
struct ScrappedOrdinance {
    format_version: String,
    root: PathBuf,
}

impl ScrappedOrdinance {
    #[allow(dead_code)]
    /// Open an existing scrapped ordinance folder
    async fn open<P: AsRef<Path>>(root: P) -> Result<Self> {
        let root = root.as_ref().to_path_buf();
        // Validate
        if !root.exists() {
            return Err(error::Error::Undefined("Path does not exist".to_string()));
        }

        let features_file = root.join("ord_db.csv");
        if !features_file.exists() {
            return Err(error::Error::Undefined(
                "Features file does not exist".to_string(),
            ));
        }

        Ok(Self {
            root,
            format_version: "0.0.1".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::ScrappedOrdinance;

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
        let target = tempfile::tempdir().unwrap();
        let ordinance_filename = target.path().join("ord_db.csv");
        let ordinance_file = std::fs::File::create(ordinance_filename).unwrap();

        ScrappedOrdinance::open(target).await.unwrap();
    }
}
