//! Parse and handle the scrapped ordinance information

mod quantitative;
mod qualitative;

use tracing::trace;

use crate::error::Result;

#[derive(Debug)]
pub(super) struct Ordinance{
    quantiative: quantitative::Quantitative,
    qualitative: qualitative::Qualitative,
}

impl Ordinance {
    pub(super) fn init_db(conn: &duckdb::Transaction) -> Result<()> {
        trace!("Initializing database for Ordinance");

        quantitative::Quantitative::init_db(conn)?;
        qualitative::Qualitative::init_db(conn)?;

        trace!("Database ready for Ordinance");
        Ok(())
    }

    /// Open the quantiative ordinance from scrapped output
    pub(super) async fn open<P: AsRef<std::path::Path>>(root: P) -> Result<Ordinance> {
        trace!("Opening quantitative ordinance of {:?}", root.as_ref());

        let ordinance = Ordinance{
            quantiative: quantitative::Quantitative::open(root.as_ref()).await?,
            qualitative: qualitative::Qualitative::open(root.as_ref()).await?,
        };

        trace!("Opened ordinance: {:?}", ordinance);

        Ok(ordinance)
    }

    pub(super) fn write(&self, conn: &duckdb::Transaction, commit_id: usize) -> Result<()> {
        trace!("Writing ordinance to database");

        self.quantiative.write(conn, commit_id)?;
        self.qualitative.write(conn, commit_id)?;

        trace!("Ordinance written to database");
        Ok(())
    }
}

#[cfg(test)]
/// Samples of quantitative ordinance to support testing
pub(crate) mod sample {
    use crate::error::Result;
    use std::io::Write;

    pub(crate) fn basic() -> String {
        let mut output = String::new();
        output.push_str("county,state,subdivison,jurisdiction_type,FIPS,feature,value,units,offset,min_dist,max_dist,summary,ord_year,section,source\n");
        output.push_str(
            "county-1,state-1,,jurisdiction_type-1,11111,feature-1,,,,,,,2001,,source-1\n",
        );
        output.push_str(
            "county-2,state-2,,jurisdiction_type-2,22222,feature-2,,,,,,,2002,,source-2\n",
        );
        output
    }

    pub(crate) fn as_file<P: AsRef<std::path::Path>>(path: P) -> Result<std::fs::File> {
        let mut file = std::fs::File::create(path)?;
        writeln!(file, "{}", basic())?;
        file.flush()?;
        Ok(file)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn dev() {
        let tmp = tempfile::tempdir().unwrap();
        sample::as_file(tmp.path()).unwrap();
        let ordinance = Ordinance::open(&tmp).await.unwrap();
    }
}
