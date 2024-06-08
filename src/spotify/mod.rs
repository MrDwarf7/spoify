pub mod auth;
pub mod library_section;
pub mod new_release_section;
pub mod player;
pub mod playlist_control;
pub mod search;
pub mod user_playlist;
pub mod user_stats;

mod error_spotify;
pub use error_spotify::SpotifyError;
pub type Result<T> = std::result::Result<T, SpotifyError>;
