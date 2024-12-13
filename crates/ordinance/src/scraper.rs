//! Support for the ordinance scrapper output

use std::path::PathBuf;

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
/// Abstraction for the ordinance scrapper raw output
///
/// The ordinance scrapper outputs a standard directory with multiple files
/// and sub-directories. This struct abstracts the access to such output.
struct ScrappedOrdinance {
    root: PathBuf,
    format_version: String,
}

impl ScrappedOrdinance {
    #[allow(dead_code)]
    /// Open an existing scrapped ordinance folder
    fn open(root: PathBuf) -> Result<Self> {
        // Validate
        if !root.exists() {
            return Err(error::Error::Undefined("Path does not exist".to_string()));
        }

        let features_file = root.join("wind_db.csv");
        if !features_file.exists() {
            return Err(error::Error::Undefined(
                "Features file does not exist".to_string(),
            ));
        }

        Ok(Self { root , format_version: "0.0.1".to_string() })
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn dev() {

        assert!(true);
    }
}
