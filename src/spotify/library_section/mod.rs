mod liked_songs;
mod made_fy;
mod made_fy_tracks;
mod podcast;
mod recently_played;
mod user_album_tracks;
mod user_albums;
mod user_artist_tracks;
mod user_artists;

pub use liked_songs::{liked_tracks, process_liked_tracks};
pub use made_fy::{made_fy, process_made_fy};
pub use made_fy_tracks::{fetch_made_fy_tracks, process_made_fy_tracks};
pub use podcast::{process_podcasts, user_podcast};
pub use recently_played::{process_recently_played, recently_played};
pub use user_album_tracks::{process_user_album_tracks, user_album_tracks};
pub use user_albums::{process_user_albums, user_albums};
pub use user_artist_tracks::{process_user_artist_tracks, user_artist_tracks};
pub use user_artists::{process_user_artists, user_artists};

// pub use super::SpotifyError;

mod error_spotify_library;
pub use error_spotify_library::SpotifyLibraryError;
pub type Result<T> = std::result::Result<T, SpotifyLibraryError>;

use crate::app::App;
use crate::spotify::auth::get_spotify_client;
use futures::FutureExt;
use futures_util::TryStreamExt;
use regex::Regex;
use rspotify::{
    model::{
        AlbumId, ArtistId, FullArtist, FullTrack, PlayHistory, PlaylistId, PlaylistItem,
        SavedAlbum, SavedTrack, Show, SimplifiedPlaylist, SimplifiedTrack,
    },
    prelude::{BaseClient, OAuthClient, *},
    ClientCredsSpotify, Credentials,
};
use serde_json::{json, Value};
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;
