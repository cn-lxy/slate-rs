use reqwest::header;

use crate::models::check_res::CheckRes;
use crate::models::login::{LoginReq, LoginRes};
use crate::models::music::MusicJSON;
use crate::models::music_url::MusicUrlJson;
use crate::models::playlist_detail::PlaylistDetail;
use crate::models::register::{RegisterReq, RegisterRes};
use crate::models::service::ServiceState;
use crate::*;

#[tauri::command]
pub async fn check_server() -> Result<ServiceState, String> {
    let url = "http://localhost:3000";
    let resp = reqwest::get(url).await.unwrap().text().await.unwrap();
    let mut ss = ServiceState {
        code: 0,
        msg: "Service is not ok!".into(),
    };
    if resp.len() != 0 {
        ss.code = 200;
        ss.msg = "Service is ok!".into();
    }
    Ok(ss)
}

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub async fn get_music_detail(id: u64) -> MusicJSON {
    let url = format!("http://localhost:3000/song/detail?ids={}", id);
    let resp = reqwest::get(url)
        .await
        .unwrap()
        .json::<MusicJSON>()
        .await
        .unwrap();
    resp.clone()
}

#[tauri::command]
pub async fn get_music_url(id: u64) -> MusicUrlJson {
    let level = "standard"; // 默认音乐质量等级为标准
    let url = format!(
        "http://localhost:3000/song/url/v1?id={}&level={}",
        id, level
    );
    let resp = reqwest::get(url)
        .await
        .unwrap()
        .json::<MusicUrlJson>()
        .await
        .unwrap();
    resp
}

#[tauri::command]
pub async fn check(token: String) -> Result<CheckRes, String> {
    let url = "http://localhost:8000/check";

    let mut headers = header::HeaderMap::new();
    let mut auth_value = header::HeaderValue::from_bytes(token.as_bytes()).unwrap();
    auth_value.set_sensitive(true);
    headers.insert(header::AUTHORIZATION, auth_value);

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
    let res = client
        .get(url)
        .send()
        .await
        .unwrap()
        .json::<CheckRes>()
        .await;
    match res {
        Ok(r) => Ok(r),
        Err(e) => Ok(CheckRes {
            code: 401,
            msg: e.to_string(),
            data: None,
        }),
    }
}

// login
#[tauri::command]
pub async fn login(req_data: LoginReq, t: String) -> Result<LoginRes, ()> {
    let url = format!("http://localhost:8000/login?t={}", t);
    
    let json_value: serde_json::Value;
    if t == "nickname" {
        json_value =  serde_json::json!({
            "nickname": req_data.username,
            "password": req_data.password,
        });
    } else if t == "email" {
        json_value =  serde_json::json!({
            "email": req_data.username,
            "password": req_data.password,
        });
    } else {
        return Ok(LoginRes {
            code: 400,
            msg: "query `t` error".into(),
            data: None,
        });
    }

    // use reqwest lib send post request and parse response data to string
    let res = reqwest::Client::new()
        .post(url)
        .json(&json_value)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("res: {}", res);
    let res = serde_json::from_str::<LoginRes>(&res).unwrap();
    Ok(res)
}

// register: the function is same and `login` function
#[tauri::command]
pub async fn register(req_data: RegisterReq) -> Result<RegisterRes, ()> {
    let url = "http://localhost:8000/register";
    let res = reqwest::Client::new()
        .post(url)
        .json(&req_data)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("res: {}", res);
    let res = serde_json::from_str::<RegisterRes>(&res).unwrap();
    Ok(res)
}

/// Get plaaylist detail
#[tauri::command]
#[allow(dead_code)]
pub async fn get_playlist_detail(id: u64) -> Result<PlaylistDetail, String> {
    let url = format!("http://localhost:3000/playlist/detail?id={}", id);
    let resp = reqwest::get(url)
        .await
        .unwrap()
        .json::<PlaylistDetail>()
        .await
        .unwrap();
    println!("{:?}", resp);
    Ok(resp)
}

/// Get the hot music list
#[allow(dead_code)]
#[tauri::command]
pub async fn get_hot_music_list(id: u64, limit: u8, offset: u8) -> Result<MusicJSON, String> {
    let url = format!("http://localhost:3000/playlist/track/all?id={}&limit={}&offset={}", id, limit, offset);
    let resp = reqwest::get(url)
        .await
        .unwrap()
        .json::<MusicJSON>()
        .await
        .unwrap();
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn test_get_music_detail() {
        let m = aw!(get_music_detail(1974443814));
        println!("{:?}", m);
    }

    #[test]
    fn test_get_music_url() {
        let id = 1974443814;
        let music_url_json = aw!(get_music_url(id));
        println!(
            "This music id is {}'s music url is: {:?}",
            id, music_url_json
        );
    }

    #[test]
    fn test_chekc_server() {
        let s = aw!(check_server());
        println!("{:?}", s);
    }

    #[test]
    fn test_login() {
        let req_data = LoginReq{
            username: "Dave".into(),
            password: "password".into(),
        };
        let res = aw!(login(req_data, "nickname".into())).unwrap();
        println!("{:?}", serde_json::to_string(&res).unwrap());
    }

    #[test]
    fn test_register() {
        let req_data = RegisterReq{
            nickname: "Dave2".into(),
            password: "password".into(),
            email: "Dave2@emai.com".into(),
        };
        let res = aw!(register(req_data)).unwrap();
        println!("{:?}", serde_json::to_string(&res).unwrap());
    }

    #[test]
    fn test_get_playlist_detail() {
        let id: u64 = 19723756;
        let res = aw!(get_playlist_detail(id));
        println!("{:?}", res);
    }

    #[test]
    fn test_get_hot_music_list() {
        let id: u64 = 19723756;
        let limit: u8 = 10;
        let offset: u8 = 0;
        let res = aw!(get_hot_music_list(id, limit, offset));
        println!("{:?}", res);
    }
}
