use serde::{Deserialize, Serialize};

// Login request struct
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginReq {
    pub username: String,
    pub password: String,
}

// Login response struct
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRes {
    pub code: u16,
    pub msg: String,
    pub data: Option<Data>,
}

// login reosponse data struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub token: String,
}

#[allow(unused_imports)]
mod tests {
    use crate::models::login::{LoginRes, Data};

    #[test]
    fn transition() {
        let login_res = LoginRes {
            code: 200,
            msg: "Login success!".into(),
            data: Some(Data {
                token: "test".into(),
            }),
        };
        let login_res_json = serde_json::to_string(&login_res).unwrap();
        println!("{:?}", login_res_json);
        let login_res_json: LoginRes = serde_json::from_str(login_res_json.as_str()).unwrap();
        println!("{:?}", login_res_json);
    }

    #[test]
    fn transition_none() {
        let login_res_string = "{\"code\":200,\"msg\":\"Login success!\"}";
        let login_res_json: LoginRes = serde_json::from_str(login_res_string).unwrap();
        println!("{:?}", login_res_json);
    }
}