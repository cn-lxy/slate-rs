use serde::{Serialize, Deserialize};

/// 歌手信息结构体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Artist {
    /// 歌手ID
    pub id: u64,
    /// 歌手名称
    pub name: String,
}
