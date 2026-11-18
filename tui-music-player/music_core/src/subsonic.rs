use reqwest::blocking::Client;
use serde::Deserialize;
use serde::de::DeserializeOwned;

use crate::config::Config;
use crate::models::{ArtistIndex, Genre, Playlist};

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
