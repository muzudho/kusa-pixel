extern crate image;

mod res;

use crate::res::image::Frame;
use image::*;
use std::path::Path;

fn main() {
    // サイズ指定で☆（＾～＾）
    {
        let frame = Frame::new(32, 64);
        image::save_buffer(
            &Path::new("output/32x64.png"),
            &frame.to_vec(),
            frame.width,
            frame.height,
            ColorType::Rgba8,
        )
        .unwrap();
    }
}
