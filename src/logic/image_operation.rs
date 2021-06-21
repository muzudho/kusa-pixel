use crate::canvas::Frame;
use image::{save_buffer, ColorType};
// use piston_window::{Flip, G2dTexture, PistonWindow, Texture, TextureSettings};
use std::path::Path;

pub fn write_frame(frame: &Frame, path: &str) {
    save_buffer(
        &Path::new(path),
        &frame.to_vec(),
        frame.width,
        frame.height,
        ColorType::Rgba8,
    )
    .unwrap();
}

/*
pub fn create_texture(png_path: &str, window: &mut PistonWindow) -> G2dTexture {
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    Texture::from_path(
        &mut window.create_texture_context(),
        &assets.join(png_path),
        Flip::None,
        &TextureSettings::new(),
    )
    .unwrap()
}
*/
