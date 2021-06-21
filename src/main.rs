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

mod canvas;
mod grid;
mod logic;
mod paint_tool;
mod pointing;
mod settings;

use crate::canvas::Frame;
use crate::logic::image_operation::*;
use crate::logic::window_operation::*;
use crate::settings::*;
use std::path::Path;

fn main() {
    // 設定ファイルを読み込もうぜ☆（＾～＾）
    let mut settings = Settings::load();

    // 画像読込を試みようぜ☆（＾～＾）？
    let mut frame = match image::open(Path::new(&settings.image_file)) {
        Ok(img) => {
            // 画像を読み込んで始まりたいぜ☆（＾～＾）
            let frame = Frame::load_image(&img);
            settings.image_width = frame.width;
            settings.image_height = frame.height;
            frame
        }
        Err(_e) => {
            // 画像が読み込めなければ、設定ファイルで指定されたサイズで新規作成☆（＾～＾）
            let frame = Frame::new(settings.image_width, settings.image_height);
            write_frame(&frame, &settings.image_file);
            frame
        }
    };

    show_window(settings, &mut frame);
}
