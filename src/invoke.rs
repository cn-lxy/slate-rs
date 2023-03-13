use crate::models::music::*;

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

#[cfg(test)]
mod tests {
    use super::get_music_detail;

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
}
