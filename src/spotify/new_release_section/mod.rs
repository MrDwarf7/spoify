pub mod new_releases;
pub mod new_releases_tracks;

use crate::app::App;
use futures::{FutureExt, TryStreamExt};
use rspotify::{
    model::{AlbumId, SimplifiedAlbum, SimplifiedTrack},
    prelude::*,
    ClientCredsSpotify, ClientError, Credentials,
};
use serde_json::{json, Value};
use std::env;
use std::{
    fs::File,
    io::{BufReader, Write},
    path::PathBuf,
};

mod error_spotify_nrs;

pub use error_spotify_nrs::SpotifyNewReleaseSectionError;
pub type Result<T> = std::result::Result<T, SpotifyNewReleaseSectionError>;
