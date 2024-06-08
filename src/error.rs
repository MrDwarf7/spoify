pub use crate::spotify::SpotifyError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic error handler: {0}")]
    Generic(String),

    #[error("An error occurred in the library module: {0}")]
    LibraryError(String),

    // #[error("An error occurred in the Spotify module: {0}")]
    // SpotifyError(String),
    #[error("An unexpected error occurred: {0}")]
    UnexpectedError(String),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    SpotifyError(#[from] SpotifyError),
}
