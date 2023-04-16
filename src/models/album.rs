use serde::{Serialize, Deserialize};

use super::{artist::Artist, music::Music};

/// Album 专辑
/// 如果是DJ节目(dj_type != 0)或者无专辑信息(single == 1)，则专辑id为0
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[allow(non_snake_case)]
pub struct Album {
    pub id: u64,
    pub name: String,
    pub picUrl : String,
}

/// AlbumDetail 专辑详情
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[allow(non_snake_case)]
pub struct AlbumDetail {
    pub album: InnerAlbum,
    pub songs: Vec<Music>,
    pub code: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[allow(non_snake_case)]
pub struct InnerAlbum {
    pub id: u64,
    pub name: String,
    pub picUrl : String,
    pub description: String,
    pub artists: Vec<Artist>,
    pub publishTime: u64,
}