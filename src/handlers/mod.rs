mod key_event;
mod util;

pub use key_event::{handle_key_event, search_input};

use crate::spotify::playlist_control::add_track_to_playlist::add_track_to_playlist;
use crate::spotify::playlist_control::playlist_follow::follow_playlist;
use crate::spotify::playlist_control::playlist_unfollow::unfollow_playlist;
use crate::spotify::user_playlist::user_playlist::{get_playlists, process_user_playlists};
use crate::{
    app::App,
    enums::{InputMode, Library, Menu, SearchMenu},
    spotify::{
        library_section::{
            liked_songs::{liked_tracks, process_liked_tracks},
            made_fy::{made_fy, process_made_fy},
            made_fy_tracks::{fetch_made_fy_tracks, process_made_fy_tracks},
            podcast::{process_podcasts, user_podcast},
            recently_played::{process_recently_played, recently_played},
            user_album_tracks::{process_user_album_tracks, user_album_tracks},
            user_albums::{process_user_albums, user_albums},
            user_artist_tracks::{process_user_artist_tracks, user_artist_tracks},
            user_artists::{process_user_artists, user_artists},
        },
        new_release_section::new_releases_tracks::{
            new_releases_tracks, process_new_releases_tracks,
        },
        player::{
            next_track::next_track, pause_playback::pause, play_playback::play,
            previous_track::previous_track, repeat::cycle_repeat, shuffle::toogle_shuffle,
            start_playback::start_playback, volume_decrease::volume_decreament,
            volume_increase::volume_increment,
        },
        search::{
            search::process_search,
            search_albums::{process_selected_album_tracks, search_selected_album_tracks},
            search_artists::{process_selected_artist_tracks, search_selected_artist_tracks},
            search_playlists::{process_selected_playlist_tracks, search_selected_playlist_tracks},
        },
        user_playlist::user_playlist_track::{fetch_playlists_tracks, process_playlist_tracks},
    },
    Result,
};
