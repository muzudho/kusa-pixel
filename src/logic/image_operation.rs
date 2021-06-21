use crate::piston_wrapper::kusa_image::KusaImage;
use image::{save_buffer, ColorType};
// use piston_window::{Flip, G2dTexture, PistonWindow, Texture, TextureSettings};
use std::path::Path;

pub fn write_frame(k_image: &KusaImage, path: &str) {
    save_buffer(
        &Path::new(path),
        &k_image.to_vec(),
        k_image.width,
        k_image.height,
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
