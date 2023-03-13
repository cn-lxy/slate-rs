use serde::{Serialize, Deserialize};

use super::artist::*;

/// 歌曲信息结构体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Music {
    /// 歌曲名称
    pub name: String,
    /// 歌曲ID
    pub id :u64,
    /// 歌曲类型
    pub t: u8,
    /// 歌手列表
    pub ar: Vec<Artist>,
    /// 付费类型
    /// 0: 免费或无版权
    /// 1: VIP 歌曲
    /// 4: 购买专辑
    /// 8: 非会员可免费播放低音质，会员可播放高音质及下载
    /// fee 为 1 或 8 的歌曲均可单独购买 2 元单曲
    pub fee: u8,
    /// 歌曲时长
    pub dt: u64,
    /// 0: 未知
    /// 1: 原曲
    /// 2: 翻唱
    pub originCoverType: u8,
}

/// 音乐详情JSON结构体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MusicJSON {
    pub songs: Vec<Music>,
    pub code: u8,
}