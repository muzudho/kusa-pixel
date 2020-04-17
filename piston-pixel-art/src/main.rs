extern crate image;
extern crate serde;
extern crate serde_json;

mod res;

use crate::res::image::Frame;
use crate::res::logic::image_operation::*;
use crate::res::settings::*;
use crate::res::window::window::*;

fn main() {
    // 設定ファイルを読み込もうぜ☆（＾～＾）
    let settings = Settings::load();

    // サイズ指定で☆（＾～＾）
    {
        let frame = Frame::new(settings.width, settings.height);
        write_frame(&frame, &settings.file);
    }

    show_window(&settings.file);
}
