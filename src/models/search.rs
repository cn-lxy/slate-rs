use serde::{Deserialize, Serialize};

use super::music::Music;

/// Result Enum
#[derive(Debug, Serialize, Deserialize)]
pub enum SearchResType {
    Song(SearchResSong),
    Album(SearchResAlbum),
    Artist(SearchResArtist),
    Playlist(SearchResPlaylist)
}

/// Song Search Result
#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResSong {
    pub code: u32,
    pub result: SongData,
}

/// Album Search Result
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResAlbum {
    pub code: u32,
    pub result: AlbumData,
}

/// Artist Search Result
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResArtist {
    pub code: u32,
    pub result: ArtistData,
}

/// Playlist Search Result
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResPlaylist {
    pub code: u32,
    pub result: PlaylistData,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct SongData {
    pub songCount: u64,
    pub songs: Vec<Music>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct AlbumData {
    pub albumCount: u64,
    pub albums: Vec<ThisAlbum>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ArtistData {
    pub artistCount: u64,
    pub artists: Vec<ThisArtist>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistData {
    pub playlistCount: u64,
    pub playlists: Vec<ThisPlaylist>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ThisAlbum {
    pub id: u64,
    pub name: String,
    pub picUrl: String,
    pub publishTime: u64,
    pub artist: ThisArtist,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "artist")]
pub struct ThisArtist {
    pub id: u64,
    pub name: String,
    pub picUrl: String,
    pub albumSize: u64,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ThisPlaylist {
    pub id: u64,
    pub name: String,
    pub coverImgUrl: String,
    pub playCount: u64,
    pub trackCount: u64,
}