extern crate serde_json;
extern crate serde_yaml;
extern crate yaml_rust;

use crate::app::App;
use crate::handlers::keybindings::{parse_keybindings, read_keybindings, set_keybindings};
use crate::handlers::theme::{read_theme, set_theme};
use crate::spotify::new_release_section::new_releases::{new_releases, process_new_releases};
use crate::spotify::player::player::{currently_playing, process_currently_playing};
use crate::spotify::user_playlist::user_playlist::{get_playlists, process_user_playlists};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Function to update the player information in a seperate thread
pub fn update_player_info(tx: mpsc::Sender<()>, app: &mut App) {
    loop {
        // Get the user's current playback
        currently_playing().unwrap();
        process_currently_playing(app);

        // Send a message to the main thread to update the UI
        if tx.send(()).is_err() {
            break;
        }

        // Wait one second before fetching playback again
        thread::sleep(Duration::from_millis(1000));
    }
}

/*
pub fn update_current_queue(tx: mpsc::Sender<()>, app: &mut App) {
    loop {
        if let Err(e) = current_queue() {
            println!("{}", e);
        }
        // Send a message to the main thread to update the UI
        if tx.send(()).is_err() {
            break;
        }

        // Wait one second before fetching playback again
        thread::sleep(Duration::from_millis(5000));
    }
}
*/
/// Function to run before starting the main app loop
pub fn startup(app: &mut App) {
    read_keybindings();
    set_keybindings(app);
    parse_keybindings(app);

    read_theme();
    set_theme(app);

    let _ = new_releases();
    process_new_releases(app);

    get_playlists();
    process_user_playlists(app);
}
