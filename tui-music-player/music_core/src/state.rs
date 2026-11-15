use crate::config::Config;
use crate::models::Playlist;

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
    pub playlists: Loadable<Vec<Playlist>>
}
