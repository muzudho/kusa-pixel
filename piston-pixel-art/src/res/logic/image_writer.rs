use crate::res::image::Frame;
use image::*;
use std::path::Path;

pub fn write_frame(frame: &Frame, path: &str) {
    image::save_buffer(
        &Path::new("assets").join(path),
        &frame.to_vec(),
        frame.width,
        frame.height,
        ColorType::Rgba8,
    )
    .unwrap();
}
