use crate::data::kusa_color::KusaColor;
use image::*;
use std::path::Path;
//use crate::piston_wrapper::kusa_image::KusaImage;
//use image::{save_buffer, ColorType};
// use piston_window::{Flip, G2dTexture, PistonWindow, Texture, TextureSettings};

/// This is a wrapped version of Piston's image library
pub struct KusaImage {
    pub k_colors: Vec<KusaColor>,
    pub width: u32,
    pub height: u32,
    // 画像の更新があれば真
    pub dirty: bool,
}
impl KusaImage {
    pub fn new(width: u32, height: u32) -> Self {
        KusaImage {
            k_colors: vec![KusaColor::default(); (width * height) as usize],
            width: width,
            height: height,
            dirty: false,
        }
    }

    pub fn load_image(img: &DynamicImage) -> Self {
        match img {
            DynamicImage::ImageRgba8(x) => {
                let width = x.dimensions().0;
                let height = x.dimensions().1;
                let mut k_image = KusaImage::new(width, height);
                let mut i = 0;
                for p in x.pixels() {
                    let col = i % width;
                    let row = i / width;
                    k_image.set_pixel(
                        col,
                        row,
                        &KusaColor {
                            r: p[0],
                            g: p[1],
                            b: p[2],
                            a: p[3],
                        },
                    );
                    i += 1;
                }
                k_image
            }
            _ => KusaImage::new(1, 1),
        }
    }

    pub fn to_rgba_vec(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        for k_color in &self.k_colors {
            vec.extend_from_slice(&k_color.to_rgba_array());
        }
        vec
    }

    pub fn to_index(col: u32, row: u32, width: u32, height: u32) -> usize {
        if width <= col || height <= row {
            panic!(
                "Out of index. width,height({},{}) col,row({},{})",
                width, height, col, row
            );
        }
        (row * width + col) as usize
    }

    pub fn set_pixel(&mut self, col: u32, row: u32, k_color: &KusaColor) {
        //println!(
        //    "Trace   | set_pixel {} {} {} {}",
        //    col, row, self.width, self.height
        //);
        self.k_colors[KusaImage::to_index(col, row, self.width, self.height)] = k_color.clone();
    }

    pub fn get_pixel(&self, col: u32, row: u32) -> &KusaColor {
        &self.k_colors[KusaImage::to_index(col, row, self.width, self.height)]
    }
}

/// 画像の保存
pub fn write_k_image(k_image: &mut KusaImage, path: &str) {
    save_buffer(
        &Path::new(path),
        &k_image.to_rgba_vec(),
        k_image.width,
        k_image.height,
        ColorType::Rgba8,
    )
    .unwrap();
    k_image.dirty = false;
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
