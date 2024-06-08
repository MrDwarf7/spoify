#[derive(thiserror::Error, Debug)]
pub enum SpotifyNewReleaseSectionError {
    #[error("SPOTIFY: Generic error handler:  {0}")]
    Generic(String),

    #[error(transparent)]
    SpotifyError(#[from] crate::spotify::SpotifyError),

    #[error(transparent)]
    SpotifyLibraryError(#[from] crate::spotify::library_section::SpotifyLibraryError),

    #[error(transparent)]
    SpotifyNRSIOError(#[from] std::io::Error),
}
