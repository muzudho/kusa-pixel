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

mod grid;
mod logic;
mod paint_tool;
mod piston_wrapper;
mod pointing;
mod settings;

use crate::logic::image_operation::*;
use crate::logic::window_operation::*;
use crate::piston_wrapper::kusa_image::KusaImage;
use crate::settings::*;
use std::path::Path;

fn main() {
    // 設定ファイルを読み込もうぜ☆（＾～＾）
    let mut settings = Settings::load();

    // Start by loading the image file
    let mut k_image = match image::open(Path::new(&settings.image_file)) {
        Ok(img) => {
            let k_image = KusaImage::load_image(&img);
            // Priority is given to the width and height of the image file rather than the configuration file
            settings.image_width = k_image.width;
            settings.image_height = k_image.height;
            k_image
        }
        Err(_e) => {
            // If there is no image file, create a new one with the size specified in the configuration file
            let k_image = KusaImage::new(settings.image_width, settings.image_height);
            write_frame(&k_image, &settings.image_file);
            k_image
        }
    };

    show_window(settings, &mut k_image);
}
