use crate::config::Config;
use crate::models::{Album, Artist, ArtistIndex, Genre, Playlist};

pub enum AuthState {
    LoggedOut,
    LoggingIn,
    LoggedIn(Config),
    Failed(String),
}

pub enum Loadable<T> {
    NotLoaded,
    Loading,
    Loaded(T),
    Failed(String),
}

pub struct AppState {
    pub auth: AuthState,
    pub playlists: Loadable<Vec<Playlist>>,
    pub artist_index: Loadable<Vec<ArtistIndex>>,
    pub genres: Loadable<Vec<Genre>>,
    pub artist: Loadable<Artist>,
    pub album: Loadable<Album>,
}
