use crate::config::Config;
use crate::models::Playlist;

pub enum Action {
    Login {
        uname: String,
        password: String,
        server: String,
    },
    LoginSucceeded(Config),
    LoginFailed(String),

    LoadPlaylists,
    PlaylistsLoaded(Vec<Playlist>),
    PlaylistsFailed(String),
}
