use serde::{Serialize, Deserialize};

use super::music::Music;

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Playlist {
    pub id: u64,
    pub name: String,
    pub coverImgUrl: String,
    pub description: String,
    pub tracks: Vec<Music>,
    pub playCount: u64,
    pub trackCount: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistDetail {
    pub code: u64,
    pub playlist: Playlist,
}