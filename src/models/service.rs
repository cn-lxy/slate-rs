use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServiceState {
    pub code: u8,
    pub msg: String,
}