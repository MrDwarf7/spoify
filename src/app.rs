use crate::enums::{InputMode, Library, Menu};
use crate::handlers::key_event::handle_events;
use crate::handlers::key_event::search_input;
use crate::ui::tui;
use crate::ui::ui::render_frame;
use ratatui::style::Color;
use ratatui::widgets::{ListState, TableState};
use std::io;

#[derive(Clone, Debug)]
pub struct App {
    pub exit: bool, //to control app's exit

    pub selected_library: Library,
    pub selected_menu: Menu,
    pub library_index: usize,

    pub library_state: ListState,
    pub can_navigate_menu: bool,

    pub search_query: String,
    pub input: String,
    pub cursor_position: usize,
    pub input_mode: InputMode,
    pub search_results_rendered: bool,

    pub album_names: Vec<String>,
    pub album_links: Vec<String>,
    pub track_names: Vec<String>,
    pub track_links: Vec<String>,
    pub playlist_names: Vec<String>,
    pub playlist_links: Vec<String>,
    pub artist_names: Vec<String>,
    pub artist_links: Vec<String>,
    pub selected_album: bool,
    pub selected_track: bool,
    pub selected_playlist: bool,
    pub selected_artist: bool,
    pub selected_search: bool,
    pub search_state: ListState,
    pub album_state: ListState,
    pub track_state: ListState,
    pub playlist_state: ListState,
    pub artist_state: ListState,

    pub user_playlist_names: Vec<String>,
    pub user_playlist_links: Vec<String>,
    pub user_playlist_track_names: Vec<String>,
    pub user_playlist_track_duration: Vec<i64>,
    pub user_playlist_artist_names: Vec<String>,
    pub user_playlist_track_links: Vec<String>,
    pub user_playlist_artist_links: Vec<String>,
    pub user_playlist_album_names: Vec<String>,
    pub current_user_playlist: String,
    pub selected_playlist_uri: String,
    pub user_playlist_display: bool,
    pub user_playlist_tracks_selected: bool,
    pub user_playlist_state: ListState,
    pub user_playlist_tracks_state: TableState,

    pub liked_song_names: Vec<String>,
    pub liked_song_links: Vec<String>,
    pub liked_song_duration: Vec<i64>,
    pub liked_song_artist_names: Vec<String>,
    pub liked_song_artist_links: Vec<String>,
    pub liked_song_album_names: Vec<String>,
    pub liked_songs_selected: bool,
    pub liked_song_display: bool,
    pub selected_liked_song_uri: String,
    pub liked_songs_state: TableState,

    pub user_album_names: Vec<String>,
    pub user_album_links: Vec<String>,
    pub user_album_artist_names: Vec<String>,
    pub user_album_artist_links: Vec<String>,
    pub user_album_tracks: Vec<usize>,
    pub user_album_selected: bool,
    pub user_album_display: bool,
    pub user_album_state: TableState,
    pub user_album_selected_uri: String,
    pub current_user_album: String,

    pub podcast_names: Vec<String>,
    pub podcast_links: Vec<String>,
    pub podcast_publisher: Vec<String>,
    pub podcast_selected: bool,
    pub podcast_display: bool,
    pub podcast_state: TableState,
    pub podcast_selected_uri: String,
    pub current_podcast: String,

    pub recently_played_names: Vec<String>,
    pub recently_played_links: Vec<String>,
    pub recently_played_duration: Vec<i64>,
    pub recently_played_artist_names: Vec<String>,
    pub recently_played_artist_links: Vec<String>,
    pub recently_played_album_names: Vec<String>,
    pub recently_played_selected: bool,
    pub recently_played_display: bool,
    pub selected_recently_played_uri: String,
    pub recently_played_state: TableState,

    pub user_artist_names: Vec<String>,
    pub user_artist_links: Vec<String>,
    pub user_artist_selected: bool,
    pub user_artist_display: bool,
    pub user_artist_state: TableState,
    pub user_artist_selected_uri: String,
    pub current_user_artist: String,

    pub current_device_name: String,
    pub current_device_volume: String,
    pub is_device_active: Vec<bool>,
    pub playback_status: String,
    pub device_ids: Vec<String>,
    pub current_device_id: Option<String>,
    pub shuffle_status: String,
    pub repeat_status: String,
    pub is_shuffle: bool,

    pub currrent_timestamp: i64,
    pub ending_timestamp: i64,
    pub currently_playing_artist: String,
    pub current_playing_name: String,
    pub current_playing_album: String,
    pub is_playing: bool,
    pub progress_bar_ratio: f64,

    pub border_color: Color,
    pub highlight_color: Color,
    pub background_color: Color,
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            // drawing the ui
            terminal.draw(|frame| render_frame(frame, self.selected_menu, self))?;
            // Handling user inputs
            if self.selected_menu == Menu::Search {
                if self.input_mode == InputMode::Editing {
                    search_input(self)?;
                } else {
                    handle_events(self)?;
                }
            } else {
                handle_events(self)?;
            }
        }
        Ok(())
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            //configure colors of ui
            highlight_color: Color::Rgb(0, 255, 146),
            border_color: Color::Rgb(29, 185, 84),
            background_color: Color::Rgb(33, 33, 33),

            exit: false,
            selected_library: Library::MadeFY,
            selected_menu: Menu::Default,
            library_index: 0,
            library_state: ListState::default(),
            user_playlist_state: ListState::default(),
            search_query: "".to_string(),
            input: String::new(),
            input_mode: InputMode::Normal,
            cursor_position: 0,
            album_names: Vec::new(),
            album_links: Vec::new(),
            track_names: Vec::new(),
            track_links: Vec::new(),
            playlist_names: Vec::new(),
            playlist_links: Vec::new(),
            artist_names: Vec::new(),
            artist_links: Vec::new(),
            search_results_rendered: false,
            user_playlist_names: Vec::new(),
            user_playlist_links: Vec::new(),
            user_playlist_track_names: Vec::new(),
            user_playlist_track_duration: Vec::new(),
            user_playlist_artist_names: Vec::new(),
            user_playlist_track_links: Vec::new(),
            user_playlist_artist_links: Vec::new(),
            selected_playlist_uri: String::new(),
            current_user_playlist: String::new(),
            user_playlist_display: false,
            user_playlist_tracks_selected: false,
            user_playlist_tracks_state: TableState::default(),
            liked_songs_state: TableState::default(),
            liked_song_names: Vec::new(),
            liked_song_links: Vec::new(),
            liked_song_duration: Vec::new(),
            liked_song_artist_names: Vec::new(),
            liked_song_artist_links: Vec::new(),
            liked_songs_selected: false,
            selected_liked_song_uri: String::new(),
            liked_song_display: false,
            user_album_display: false,
            user_playlist_album_names: Vec::new(),
            liked_song_album_names: Vec::new(),
            album_state: ListState::default(),
            track_state: ListState::default(),
            playlist_state: ListState::default(),
            artist_state: ListState::default(),
            selected_album: false,
            selected_track: false,
            selected_playlist: false,
            selected_artist: false,
            selected_search: false,
            user_album_selected: false,
            search_state: ListState::default(),
            user_album_state: TableState::default(),
            user_album_names: Vec::new(),
            user_album_links: Vec::new(),
            user_album_selected_uri: String::new(),
            current_user_album: String::new(),
            user_album_artist_names: Vec::new(),
            user_album_artist_links: Vec::new(),
            user_album_tracks: Vec::new(),
            can_navigate_menu: true,

            recently_played_names: Vec::new(),
            recently_played_links: Vec::new(),
            recently_played_duration: Vec::new(),
            recently_played_artist_names: Vec::new(),
            recently_played_artist_links: Vec::new(),
            recently_played_album_names: Vec::new(),
            recently_played_selected: false,
            recently_played_display: false,
            selected_recently_played_uri: String::new(),
            recently_played_state: TableState::default(),

            podcast_names: Vec::new(),
            podcast_links: Vec::new(),
            podcast_publisher: Vec::new(),
            podcast_selected: false,
            podcast_display: false,
            podcast_state: TableState::default(),
            podcast_selected_uri: String::new(),
            current_podcast: String::new(),

            user_artist_names: Vec::new(),
            user_artist_links: Vec::new(),
            user_artist_selected: false,
            user_artist_display: false,
            user_artist_state: TableState::default(),
            user_artist_selected_uri: String::new(),
            current_user_artist: String::new(),

            current_device_name: String::new(),
            current_device_volume: String::new(),
            is_device_active: Vec::new(),
            playback_status: String::from("Playing"),
            shuffle_status: String::from("Off"),
            repeat_status: String::from("Off"),
            is_shuffle: false,
            device_ids: Vec::new(),
            current_device_id: Some(String::new()),

            currrent_timestamp: i64::from(0),
            ending_timestamp: i64::from(1),
            currently_playing_artist: String::new(),
            current_playing_name: String::new(),
            current_playing_album: String::new(),
            is_playing: false,
            progress_bar_ratio: 0.0,
        }
    }
}
