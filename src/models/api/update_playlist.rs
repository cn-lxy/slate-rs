use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePlaylistJSON {
    pub code: u64,
    pub msg: String
}