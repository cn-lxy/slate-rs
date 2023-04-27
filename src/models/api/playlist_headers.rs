use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistHeadersJSON {
    pub msg: String,
    pub data: Option<Vec<PlaylistHeader>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct PlaylistHeader {
    pub id: u64,
    pub name: String,
    pub createTime: String,
    pub updateTime: String,
    pub playCount: u64,
}
