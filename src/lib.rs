//! A pixel art painter for people who are sick of GUIs.

// Publish:
//
// (1) Version up on Cargo.toml.
// (2) `cargo run --example example1`
// (3) `cargo doc --open`
// (4) Comit to Git-hub.
// (5) `cargo publish --dry-run`
// (6) `cargo publish`

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

mod data;
mod grid;
mod paint_tool;
mod piston_wrapper;
mod settings;
mod window_operation;

use crate::piston_wrapper::kusa_image::write_k_image;
use crate::piston_wrapper::kusa_image::KusaImage;
use crate::settings::*;
use crate::window_operation::*;
use std::path::Path;

pub fn run() {
    // 構成（＾～＾）
    let app = KusaApp::default();

    // 設定ファイルを読み込もうぜ☆（＾～＾）
    let mut settings = match Settings::load(&app.settings_path) {
        Ok(x) => x,
        Err(_) => {
            let settings = Settings::default();
            settings.save(&app.settings_example_path);
            settings
        }
    };

    println!("Debug   | Load image {}", settings.image_file);
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
            let mut k_image = KusaImage::new(settings.image_width, settings.image_height);
            write_k_image(&mut k_image, &settings.image_file);
            k_image
        }
    };

    show_window(&app, settings, &mut k_image);
}

pub struct KusaApp {
    settings_path: String,
    settings_example_path: String,
}
impl Default for KusaApp {
    fn default() -> Self {
        KusaApp {
            settings_path: "settings.json".to_string(),
            settings_example_path: "settings-example.json".to_string(),
        }
    }
}
