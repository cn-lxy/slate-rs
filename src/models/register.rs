use serde::{Deserialize, Serialize};

// register request struct
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterReq {
    pub nickname: String,
    pub email: String,
    pub password: String,
}


// register response struct
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRes {
    pub code: u16,
    pub msg: String,
    pub data: Option<Data>,
}

// register response data struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub token: String,
}