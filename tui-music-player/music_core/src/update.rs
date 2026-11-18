use std::sync::mpsc::Sender;
use std::thread;

use crate::action::Action;
use crate::state::{AppState, AuthState, Loadable};
use crate::{auth, config, subsonic};

pub fn update(state: &mut AppState, action: Action, tx: &Sender<Action>) {
    match action {
        Action::Login { uname, password, server } => {
            state.auth = AuthState::LoggingIn;
            let tx = tx.clone();
            thread::spawn(move || {
                let result = match auth::login(&uname, &password, &server) {
                    Ok(cfg) => Action::LoginSucceeded(cfg),
                    Err(msg) => Action::LoginFailed(msg),
                };
                let _ = tx.send(result);
            });
        }
        Action::LoginSucceeded(cfg) => match config::save(&cfg) {
            Ok(()) => state.auth = AuthState::LoggedIn(cfg),
            Err(e) => state.auth = AuthState::Failed(format!("Could not save login: {e}")),
        },
        Action::LoginFailed(msg) => state.auth = AuthState::Failed(msg),
        Action::LoadPlaylists => {
            if let AuthState::LoggedIn(cfg) = &state.auth {
                state.playlists = Loadable::Loading;
                let cfg = cfg.clone();
                let tx = tx.clone();
                thread::spawn(move || {
                    let result = match subsonic::get_playlists(&cfg) {
                        Ok(list) => Action::PlaylistsLoaded(list),
                        Err(msg) => Action::PlaylistsFailed(msg)
                    };
                    let _ = tx.send(result);
                });
            }
        }
        Action::PlaylistsLoaded(list) => state.playlists = Loadable::Loaded(list),
        Action::PlaylistsFailed(msg) => state.playlists = Loadable::Failed(msg),

        Action::LoadArtists => {
            if let AuthState::LoggedIn(cfg) = &state.auth {
                state.artist_index = Loadable::Loading;
                let cfg = cfg.clone();
                let tx = tx.clone();
                thread::spawn(move || {
                    let result = match subsonic::get_artists(&cfg) {
                        Ok(list) => Action::ArtistsLoaded(list),
                        Err(msg) => Action::ArtistsFailed(msg)
                    };
                    let _ = tx.send(result);
                });
            }
        }
        Action::ArtistsLoaded(list) => state.artist_index = Loadable::Loaded(list),
        Action::ArtistsFailed(msg) => state.artist_index = Loadable::Failed(msg),

        Action::LoadArtist(id) => {
            if let AuthState::LoggedIn(cfg) = &state.auth {
                state.artist = Loadable::Loading;
                let cfg = cfg.clone();
                let tx = tx.clone();
                thread::spawn(move || {
                    let result = match subsonic::get_artist(&cfg, &id) {
                        Ok(artist) => Action::ArtistLoaded(artist),
                        Err(msg) => Action::ArtistFailed(msg)
                    };
                    let _ = tx.send(result);
                });
            }
        }
        Action::ArtistLoaded(artist) => state.artist = Loadable::Loaded(artist),
        Action::ArtistFailed(msg) => state.artist = Loadable::Failed(msg),

        Action::LoadAlbum(id) => {
            if let AuthState::LoggedIn(cfg) = &state.auth {
                state.album = Loadable::Loading;
                let cfg = cfg.clone();
                let tx = tx.clone();
                thread::spawn(move || {
                    let result = match subsonic::get_album(&cfg, &id) {
                        Ok(album) => Action::AlbumLoaded(album),
                        Err(msg) => Action::AlbumFailed(msg)
                    };
                    let _ = tx.send(result);
                });
            }
        }
        Action::AlbumLoaded(album) => state.album = Loadable::Loaded(album),
        Action::AlbumFailed(msg) => state.album = Loadable::Failed(msg),

        Action::LoadPlaylist(id) => {
            if let AuthState::LoggedIn(cfg) = &state.auth {
                state.playlist = Loadable::Loading;
                let cfg = cfg.clone();
                let tx = tx.clone();
                thread::spawn(move || {
                    let result = match subsonic::get_playlist(&cfg, &id) {
                        Ok(playlist) => Action::PlaylistLoaded(playlist),
                        Err(msg) => Action::PlaylistFailed(msg)
                    };
                    let _ = tx.send(result);
                });
            }
        }
        Action::PlaylistLoaded(playlist) => state.playlist = Loadable::Loaded(playlist),
        Action::PlaylistFailed(msg) => state.playlist = Loadable::Failed(msg),

        Action::LoadSongsByGenre(genre) => {
            if let AuthState::LoggedIn(cfg) = &state.auth {
                state.genre_songs = Loadable::Loading;
                let cfg = cfg.clone();
                let tx = tx.clone();
                thread::spawn(move || {
                    let result = match subsonic::get_songs_by_genre(&cfg, &genre, 500, 0) {
                        Ok(songs) => Action::SongsByGenreLoaded(songs),
                        Err(msg) => Action::SongsByGenreFailed(msg)
                    };
                    let _ = tx.send(result);
                });
            }
        }
        Action::SongsByGenreLoaded(songs) => state.genre_songs = Loadable::Loaded(songs),
        Action::SongsByGenreFailed(msg) => state.genre_songs = Loadable::Failed(msg),

        Action::LoadAlbumList(list_type) => {
            if let AuthState::LoggedIn(cfg) = &state.auth {
                state.album_list = Loadable::Loading;
                let cfg = cfg.clone();
                let tx = tx.clone();
                thread::spawn(move || {
                    let result = match subsonic::get_album_list2(&cfg, &list_type, 500, 0) {
                        Ok(albums) => Action::AlbumListLoaded(albums),
                        Err(msg) => Action::AlbumListFailed(msg)
                    };
                    let _ = tx.send(result);
                });
            }
        }
        Action::AlbumListLoaded(albums) => state.album_list = Loadable::Loaded(albums),
        Action::AlbumListFailed(msg) => state.album_list = Loadable::Failed(msg),

        Action::LoadGenres => {
            if let AuthState::LoggedIn(cfg) = &state.auth {
                state.genres = Loadable::Loading;
                let cfg = cfg.clone();
                let tx = tx.clone();
                thread::spawn(move || {
                    let result = match subsonic::get_genres(&cfg) {
                        Ok(list) => Action::GenresLoaded(list),
                        Err(msg) => Action::GenresFailed(msg)
                    };
                    let _ = tx.send(result);
                });
            }
        }
        Action::GenresLoaded(list) => state.genres = Loadable::Loaded(list),
        Action::GenresFailed(msg) => state.genres = Loadable::Failed(msg)
    }
}
