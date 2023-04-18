use serde::{Serialize, Deserialize};

use super::music::Music;

/// 歌手信息结构体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Artist {
    /// 歌手ID
    pub id: u64,
    /// 歌手名称
    pub name: String,
}

/// 歌手一般信息：歌手信息（音乐数、专辑数、简介、头像、名字、id、发布时间）、50首热门歌曲
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ArtistCommonInfo {
    pub code: u64,
    pub artist: InnerArtist,
    pub hotSongs: Vec<Music>,
}

/// 歌手一般信息对应的歌手信息结构体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct InnerArtist {
    pub id: u64,
    pub name: String,
    pub picUrl: String,
    pub img1v1Url: String,
    pub briefDesc: String,
    pub musicSize: u64,
    pub albumSize: u64,
}

/// 歌手所有歌曲
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ArtistAllSongs {
    pub code: u64,
    pub more: bool,
    pub total: u64,
    pub songs: Vec<Music>,
}