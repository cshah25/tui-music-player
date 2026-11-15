use reqwest::blocking::Client;
use serde::Deserialize;

use crate::config::Config;
use crate::models::Playlist;

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

// These structs mirror the JSON layout:
// { "subsonic-response": { "status": "ok", "playlists": { "playlist": [ ... ] } } }
#[derive(Deserialize)]
struct Envelope {
    #[serde(rename = "subsonic-response")]
    response: PlaylistsResponse,
}

#[derive(Deserialize)]
struct PlaylistsResponse {
    status: String,
    playlists: Option<PlaylistList>,
    error: Option<ApiError>,
}

#[derive(Deserialize)]
struct PlaylistList {
    #[serde(default)]
    playlist: Vec<Playlist>,
}

#[derive(Deserialize)]
struct ApiError {
    message: String,
}

pub fn get_playlists(config: &Config) -> Result<Vec<Playlist>, String> {
    let body = request(config, "getPlaylists", &[])?;
    let parsed: Envelope =
        serde_json::from_str(&body).map_err(|e| format!("Could not read playlists: {e}"))?;

    let res = parsed.response;
    if res.status != "ok" {
        let msg = res.error.map(|e| e.message).unwrap_or("Unknown error".to_string());
        return Err(msg);
    }

    Ok(res.playlists.map(|p| p.playlist).unwrap_or_default())
}
