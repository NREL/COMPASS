/// Possible errors

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    DuckDBError(#[from] duckdb::Error),

    #[allow(dead_code)]
    #[error("Undefined error")]
    // Used during development while it is not clear a category of error
    // or when it is not worth to create a new error type.
    /// Undefined error
    Undefined(String),
}

pub(crate) type Result<T> = core::result::Result<T, Error>;
