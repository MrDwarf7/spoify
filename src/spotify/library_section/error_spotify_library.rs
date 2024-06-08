#[derive(thiserror::Error, Debug)]
pub enum SpotifyLibraryError {
    #[error("SPOTIFY: Generic error handler:  {0}")]
    Generic(String),

    #[error("SPOTIFY: An error occurred in the library module")]
    StreamingError {
        #[from]
        source: reqwest::Error,
    },

    #[error(transparent)]
    SpotifyAuthClientFailure(#[from] rspotify::ClientError),

    #[error(transparent)]
    SpotifyLibraryError(#[from] crate::spotify::SpotifyError),

    #[error(transparent)]
    LibraryIOError(#[from] std::io::Error),
}
