#[derive(thiserror::Error, Debug)]
pub enum SpotifyError {
    #[error("SPOTIFY: Generic error handler:  {0}")]
    Generic(String),

    #[error("SPOTIFY: An error occurred in the library module")]
    StreamingError {
        #[from]
        source: reqwest::Error,
    },

    #[error(transparent)]
    SpotifyAuthClientFailure(#[from] rspotify::ClientError),

    #[error("Failed to open user_album_tracks.json")]
    UserAlbumTracksError {
        #[from]
        source: std::io::Error,
    },

    #[error(transparent)]
    SpotifyLibraryError(#[from] Box<super::library_section::SpotifyLibraryError>),
}
