use crate::config::Config;
use crate::models::{ArtistIndex, Genre, Playlist};

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

    LoadGenres,
    GenresLoaded(Vec<Genre>),
    GenresFailed(String),
}
