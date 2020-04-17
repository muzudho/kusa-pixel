extern crate camera_controllers;
extern crate find_folder;
extern crate gfx;
extern crate gfx_device_gl;
extern crate image;
extern crate piston_window;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate shader_version;
extern crate vecmath;

mod res;

use crate::res::image::Frame;
use crate::res::logic::image_operation::*;
use crate::res::logic::window_operation::*;
use crate::res::settings::Settings;

fn main() {
    // 設定ファイルを読み込もうぜ☆（＾～＾）
    let settings = Settings::load();

    // サイズ指定で☆（＾～＾）
    {
        let frame = Frame::new(settings.width, settings.height);
        write_frame(&frame, &settings.file);
    }

    show_window(&settings);
}
