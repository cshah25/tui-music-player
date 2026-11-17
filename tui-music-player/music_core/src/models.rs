use serde::Deserialize;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub song_count: u32,
    #[serde(default)]
    pub duration: u32,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub album_count: u32,
    pub cover_art: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct ArtistIndex {
    pub name: String,
    #[serde(default, rename = "artist")]
    pub artists: Vec<Artist>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Genre {
    #[serde(default)]
    pub song_count: u32,
    #[serde(default)]
    pub album_count: u32,
    pub value: String
}
