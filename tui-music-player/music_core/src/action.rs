use crate::config::Config;
use crate::models::{Album, Artist, ArtistIndex, Genre, Playlist};

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

    LoadArtist(String), // artist id
    ArtistLoaded(Artist),
    ArtistFailed(String),

    LoadAlbum(String), // album id
    AlbumLoaded(Album),
    AlbumFailed(String),

    LoadPlaylist(String), // playlist id
    PlaylistLoaded(Playlist),
    PlaylistFailed(String),

    LoadGenres,
    GenresLoaded(Vec<Genre>),
    GenresFailed(String),
}
