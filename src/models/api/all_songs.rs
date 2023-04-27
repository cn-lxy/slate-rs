use serde::{Deserialize, Serialize};

use super::playlist_headers::PlaylistHeader;

#[derive(Debug, Serialize, Deserialize)]
pub struct AllSongsJSON {
    pub code: u64,
    pub data: AllSongsInnerData,
    pub msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct AllSongsInnerData {
    pub playlist: PlaylistHeader,
    pub songIds: Vec<u64>,
}