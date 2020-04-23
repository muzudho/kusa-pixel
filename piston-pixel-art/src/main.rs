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
use std::path::Path;

fn main() {
    // 設定ファイルを読み込もうぜ☆（＾～＾）
    let mut settings = Settings::load();

    // 画像読込を試みようぜ☆（＾～＾）？
    let mut frame = match image::open(Path::new("assets").join(&settings.file)) {
        Ok(img) => {
            // 画像を読み込んで始まりたいぜ☆（＾～＾）
            let frame = Frame::load_image(&img);
            settings.width = frame.width;
            settings.height = frame.height;
            frame
        }
        Err(_e) => {
            // 画像が読み込めなければ、設定ファイルで指定されたサイズで新規作成☆（＾～＾）
            let frame = Frame::new(settings.width, settings.height);
            write_frame(&frame, &settings.file);
            frame
        }
    };

    show_window(settings, &mut frame);
}
