//! 設定ファイル
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct Settings {
    // 画像ファイルパス。assetsディレクトリーの下から。
    pub file: String,
    // 画像ファイルの横幅
    pub width: u32,
    // 画像ファイルの縦幅
    pub height: u32,
    // 拡大率。
    pub zoom: u8,
}
impl Settings {
    /// 設定ファイル読込。
    pub fn load() -> Self {
        let path = Path::new("./input/settings.json");

        let mut file = match File::open(path) {
            Ok(x) => x,
            Err(err) => panic!("File open error. {:?}", err),
        };

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(x) => x,
            Err(err) => panic!("File open error. {:?}", err),
        };

        match serde_json::from_str(&contents) {
            Ok(x) => x,
            Err(err) => panic!("Unexpected settings: {}", err),
        }
    }
}
