//! 設定ファイル
use crate::data::kusa_color::KusaColor;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct Settings {
    // 画像ファイルパス
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
    // セルの横幅、縦幅
    pub canvas_cell_size: f64,
    // グリッドの太さ
    pub canvas_grid_thickness: f64,
    // グリッドの色
    pub canvas_grid_color: [f32; 4],
    // 描画ツールの種類
    pub paint_tool: String,
    // 描画ツールのペン先の種類
    pub paint_nib: String,
    // 描画色
    pub paint_color: KusaColor,
    // 描画ツールの線の太さ
    pub paint_thickness: f64,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            image_file: "./work/image.png".to_string(),
            image_width: 64,
            image_height: 32,
            canvas_zoom: 1.0,
            canvas_margin_top: 8.0,
            canvas_margin_right: 8.0,
            canvas_margin_bottom: 8.0,
            canvas_margin_left: 8.0,
            canvas_cell_size: 8.0,
            canvas_grid_thickness: 0.5,
            canvas_grid_color: [0.0, 0.0, 0.0, 1.0],
            paint_tool: "Pen".to_string(),
            paint_nib: "Circle".to_string(),
            paint_color: KusaColor {
                r: 230,
                g: 230,
                b: 180,
                a: 255,
            },
            paint_thickness: 1.0,
        }
    }
}
impl Settings {
    /// 設定ファイル読込
    pub fn load(settings_path: &str) -> Result<Self, String> {
        let path = Path::new(settings_path);

        let mut file = match File::open(path) {
            Ok(x) => x,
            Err(why) => return Err(format!("{}", why)),
        };

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(x) => x,
            Err(why) => return Err(format!("{}", why)),
        };

        match serde_json::from_str(&contents) {
            Ok(x) => return Ok(x),
            Err(why) => return Err(format!("Settings load fail. {}", why)),
        }
    }

    /// 設定ファイル書出し
    pub fn save(&self, settings_path: &str) {
        let path = Path::new(settings_path);
        let mut file = match OpenOptions::new().write(true).create(true).open(path) {
            Ok(file) => file,
            Err(err) => panic!("Log file open error. {:?}", err),
        };
        let json_str = match serde_json::to_string(self) {
            Ok(x) => x,
            Err(why) => panic!("couldn't serialize settings to json. : {}", &why),
        };
        if let Err(why) = file.write_all(json_str.as_bytes()) {
            panic!("couldn't write log. : {}", &why)
        }
    }
}
