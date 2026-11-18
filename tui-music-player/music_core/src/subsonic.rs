use reqwest::blocking::Client;
use serde::Deserialize;
use serde::de::DeserializeOwned;

use crate::config::Config;
use crate::models::{Album, AlbumListType, Artist, ArtistIndex, Genre, Playlist, Song};

// Shared request parsing
fn request(config: &Config, endpoint: &str, extra: &[(&str, &str)]) -> Result<String, String> {
    let mut params = vec![
        ("u", config.uname.as_str()),
        ("t", config.token.as_str()),
        ("s", config.salt.as_str()),
        ("v", "1.16.1"),
        ("c", "tui-music-player"),
        ("f", "json"),
    ];
    params.extend_from_slice(extra);

    Client::new()
        .post(format!("{}/rest/{}.view", config.server_url, endpoint))
        .form(&params)
        .send()
        .map_err(|e| format!("Could not reach server: {e}"))?
        .text()
        .map_err(|e| format!("Bad response from server: {e}"))
}

// Every response looks like:
// { "subsonic-response": {"status": "...", "error" : {...}?, <endpoint data> } }

#[derive(Deserialize)]
struct Envelope<T> {
    #[serde(rename = "subsonic-response")]
    response: Response<T>,
}

#[derive(Deserialize)]
struct Response<T> {
    status: String,
    error: Option<ApiError>,
    #[serde(flatten)]
    data: T,
}

#[derive(Deserialize)]
struct ApiError {
    message: String,
}

fn call<T: DeserializeOwned> (
    config: &Config,
    endpoint: &str,
    extra: &[(&str, &str)],
) -> Result<T, String> {
    let body = request(config, endpoint, extra)?;
    let parsed: Envelope<T> = serde_json::from_str(&body)
        .map_err(|e| format!("Could not read {endpoint} response: {e}"))?;
    let res = parsed.response;
    if res.status != "ok" {
        return Err(res.error.map(|e| e.message).unwrap_or_else(|| "Unknown error".to_string()));
    }

    Ok(res.data)
}

// getPlaylists

#[derive(Deserialize)]
struct PlaylistsBody {
    playlists: Option<PlaylistList>,
}

#[derive(Deserialize)]
struct PlaylistList {
    #[serde(default)]
    playlist: Vec<Playlist>,
}

pub fn get_playlists(config: &Config) -> Result<Vec<Playlist>, String> {
    let body: PlaylistsBody = call(config, "getPlaylists", &[])?;
    Ok(body.playlists.map(|p| p.playlist).unwrap_or_default())
}

// getArtists

#[derive(Deserialize)]
struct ArtistsBody {
    artists: Option<ArtistsList>,
}

#[derive(Deserialize)]
struct ArtistsList {
    #[serde(default)]
    index: Vec<ArtistIndex>,
}
pub fn get_artists(config: &Config) -> Result<Vec<ArtistIndex>, String> {
    let body: ArtistsBody = call(config, "getArtists", &[])?;
    Ok(body.artists.map(|a| a.index).unwrap_or_default())
}

#[derive(Deserialize)]
struct ArtistBody {
    artist: Artist,
}

pub fn get_artist(config: &Config, id: &str) -> Result<Artist, String> {
    let body: ArtistBody = call(config, "getArtist", &[("id", id)])?;
    Ok(body.artist)
}

// getAlbum

#[derive(Deserialize)]
struct AlbumBody {
    album: Album,
}

pub fn get_album(config: &Config, id: &str) -> Result<Album, String> {
    let body: AlbumBody = call(config, "getAlbum", &[("id", id)])?;
    Ok(body.album)
}

// getPlaylist

#[derive(Deserialize)]
struct PlaylistBody {
    playlist: Playlist,
}

pub fn get_playlist(config: &Config, id: &str) -> Result<Playlist, String> {
    let body: PlaylistBody = call(config, "getPlaylist", &[("id", id)])?;
    Ok(body.playlist)
}

// getGenres

#[derive(Deserialize)]
struct GenresBody {
    genres: Option<GenresList>,
}

#[derive(Deserialize)]
struct GenresList {
    #[serde(default)]
    genre: Vec<Genre>
}

pub fn get_genres(config: &Config) -> Result<Vec<Genre>, String> {
    let body: GenresBody = call(config, "getGenres", &[])?;
    Ok(body.genres.map(|g| g.genre).unwrap_or_default())
}

// getSongsByGenre

#[derive(Deserialize)]
struct SongsByGenreBody {
    #[serde(rename = "songsByGenre")]
    songs_by_genre: Option<SongList>,
}

#[derive(Deserialize)]
struct SongList {
    #[serde(default)]
    song: Vec<Song>,
}

/// `count` is capped at 500 by the server; page with `offset` for more.
pub fn get_songs_by_genre(config: &Config, genre: &str, count: u32, offset: u32) -> Result<Vec<Song>, String> {
    let count = count.to_string();
    let offset = offset.to_string();
    let body: SongsByGenreBody = call(
        config,
        "getSongsByGenre",
        &[("genre", genre), ("count", &count), ("offset", &offset)],
    )?;
    Ok(body.songs_by_genre.map(|s| s.song).unwrap_or_default())
}

// getAlbumList2

#[derive(Deserialize)]
struct AlbumList2Body {
    #[serde(rename = "albumList2")]
    album_list2: Option<AlbumList>,
}

#[derive(Deserialize)]
struct AlbumList {
    #[serde(default)]
    album: Vec<Album>,
}

/// `size` is capped at 500 by the server; page with `offset` for more.
pub fn get_album_list2(config: &Config, list_type: &AlbumListType, size: u32, offset: u32) -> Result<Vec<Album>, String> {
    let mut params = list_type.params();
    params.push(("size", size.to_string()));
    params.push(("offset", offset.to_string()));
    let extra: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
    let body: AlbumList2Body = call(config, "getAlbumList2", &extra)?;
    Ok(body.album_list2.map(|a| a.album).unwrap_or_default())
}
