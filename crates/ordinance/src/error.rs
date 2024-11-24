/// Possible errors

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
