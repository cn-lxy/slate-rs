use serde::{Serialize, Deserialize};

/// Album 专辑
/// 如果是DJ节目(dj_type != 0)或者无专辑信息(single == 1)，则专辑id为0
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[allow(non_snake_case)]
pub struct Album {
    pub id: u64,
    pub name: String,
    pub picUrl : String,
}