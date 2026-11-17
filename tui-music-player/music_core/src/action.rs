use crate::config::Config;
use crate::models::{ArtistIndex, Playlist};

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

    LoadArtists,
    ArtistsLoaded(Vec<ArtistIndex>),
    ArtistsFailed(String),
}
