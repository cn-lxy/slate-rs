use crate::models::music::MusicJSON;
use crate::models::music_url::MusicUrlJson;
use crate::models::service::ServiceState;
use crate::*;

#[tauri::command]
pub async fn chekc_server() -> Result<ServiceState, String> {
    let url = "http://localhost:3000";
    let resp = reqwest::get(url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
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
        let s = aw!(chekc_server());
        println!("{:?}", s);
    }
}
