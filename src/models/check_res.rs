use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Data {
    pub id: u64,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct CheckRes {
    pub code: u64,
    pub msg: String,
    pub data: Option<Data>,
}
