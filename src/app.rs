use crate::enums::{InputMode, Library, Menu};
use crate::handlers::key_event::handle_events;
use crate::handlers::key_event::search_input;
use crate::ui::tui;
use crate::ui::ui::render_frame;
use ratatui::style::Color;
use ratatui::widgets::{ListState, TableState};
use std::io;

pub struct App {
    pub exit: bool, //to control app's exit

    pub selected_library: Library,
    pub selected_menu: Menu,
    pub library_index: usize,
    pub library_state: ListState,
    pub user_playlist_state: ListState,
    pub user_playlist_tracks_state: TableState,
    pub liked_songs_state: TableState,
    pub user_playlist_tracks_selected: bool,
    pub liked_songs_selected: bool,

    pub search_query: String,
    pub input: String,
    pub cursor_position: usize,
    pub input_mode: InputMode,

    pub album_names: Vec<String>,
    pub album_links: Vec<String>,
    pub track_names: Vec<String>,
    pub track_links: Vec<String>,
    pub playlist_names: Vec<String>,
    pub playlist_links: Vec<String>,
    pub artist_names: Vec<String>,
    pub artist_links: Vec<String>,

    pub search_results_rendered: bool,

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

    pub liked_song_names: Vec<String>,
    pub liked_song_links: Vec<String>,
    pub liked_song_duration: Vec<i64>,
    pub liked_song_artist_names: Vec<String>,
    pub liked_song_artist_links: Vec<String>,
    pub liked_song_album_names: Vec<String>,

    pub selected_liked_song_uri: String,
    pub liked_song_display: bool,

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
            user_playlist_album_names: Vec::new(),
            liked_song_album_names: Vec::new(),
            border_color: Color::Rgb(69, 126, 89),
            highlight_color: Color::Rgb(29, 185, 84),
            background_color: Color::Rgb(33, 33, 33),
        }
    }
}
