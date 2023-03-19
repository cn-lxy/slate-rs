// music url json

use serde::{Serialize, Deserialize};


/// 获取music url 响应的结构体信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[allow(non_snake_case)]
pub struct MusicUrlJson {
    pub data: Vec<MusicUrl>,
    pub code: u8,
}

/// music url 中的data结构体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MusicUrl {
    pub id: u64,
    pub url: String,
}