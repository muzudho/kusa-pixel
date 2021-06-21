//! 設定ファイル
use crate::data::kusa_color::KusaColor;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct Settings {
    // 画像ファイルパス。assetsディレクトリーの下から
    pub image_file: String,
    // 画像ファイルの横幅
    pub image_width: u32,
    // 画像ファイルの縦幅
    pub image_height: u32,
    // 拡大率
    pub canvas_zoom: f64,
    // クライアント領域の枠からのオフセット
    pub canvas_margin_top: f64,
    pub canvas_margin_right: f64,
    pub canvas_margin_bottom: f64,
    pub canvas_margin_left: f64,
    // セルの横幅
    pub canvas_dot_width: f64,
    // セルの縦幅
    pub canvas_dot_height: f64,
    // グリッドの太さ
    pub canvas_grid_thickness: f64,
    // グリッドの色
    pub canvas_grid_color: [f32; 4],
    // 描画ツールの種類
    pub paint_tool: String,
    // 描画色
    pub paint_color: KusaColor,
}

impl Settings {
    /// 設定ファイル読込。
    pub fn load() -> Self {
        let path = Path::new("./settings.json");

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
