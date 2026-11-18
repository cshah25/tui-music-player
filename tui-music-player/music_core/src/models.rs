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
    #[serde(default, rename = "entry")]
    pub songs: Vec<Song>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub album_count: u32,
    pub cover_art: Option<String>,
    #[serde(default,rename="album")]
    pub albums: Vec<Album>,
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

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Song {
    pub id: String,
    pub parent: Option<String>,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub album: String,
    #[serde(default)]
    pub artist: String,
    pub album_id: Option<String>,
    pub artist_id: Option<String>,
    #[serde(default)]
    pub is_dir: bool,
    pub cover_art: Option<String>,
    pub track: Option<u32>,
    pub disc_number: Option<u32>,
    pub year: Option<u32>,
    pub genre: Option<String>,
    #[serde(default)]
    pub duration: u32,
    #[serde(default)]
    pub bit_rate: u32,
    #[serde(default)]
    pub size: u64,
    pub suffix: Option<String>,
    pub content_type: Option<String>,
    #[serde(default)]
    pub is_video: bool,
    pub path: Option<String>,
    pub created: Option<String>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub artist: String,
    pub artist_id: Option<String>,
    pub cover_art: Option<String>,
    #[serde(default)]
    pub song_count: u32,
    #[serde(default)]
    pub duration: u32,
    pub year: Option<u32>,
    pub genre: Option<String>,
    #[serde(default)]
    pub created: String,
    #[serde(default, rename = "song")]
    pub songs: Vec<Song>,
}

/// The `type` param for getAlbumList2, plus any extra params that type requires.
#[derive(Clone)]
pub enum AlbumListType {
    Random,
    Newest,
    Highest,
    Frequent,
    Recent,
    AlphabeticalByName,
    AlphabeticalByArtist,
    Starred,
    ByYear { from: u32, to: u32 },
    ByGenre(String),
}

impl AlbumListType {
    pub fn params(&self) -> Vec<(&'static str, String)> {
        match self {
            AlbumListType::Random => vec![("type", "random".into())],
            AlbumListType::Newest => vec![("type", "newest".into())],
            AlbumListType::Highest => vec![("type", "highest".into())],
            AlbumListType::Frequent => vec![("type", "frequent".into())],
            AlbumListType::Recent => vec![("type", "recent".into())],
            AlbumListType::AlphabeticalByName => vec![("type", "alphabeticalByName".into())],
            AlbumListType::AlphabeticalByArtist => vec![("type", "alphabeticalByArtist".into())],
            AlbumListType::Starred => vec![("type", "starred".into())],
            AlbumListType::ByYear { from, to } => vec![
                ("type", "byYear".into()),
                ("fromYear", from.to_string()),
                ("toYear", to.to_string()),
            ],
            AlbumListType::ByGenre(genre) => vec![
                ("type", "byGenre".into()),
                ("genre", genre.clone()),
            ],
        }
    }
}
