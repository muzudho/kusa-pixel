use image::*;

// This is a wrapped version of Piston's image library
pub struct KusaImage {
    pub dots: Vec<Dot>,
    pub width: u32,
    pub height: u32,
}
impl KusaImage {
    pub fn new(width: u32, height: u32) -> Self {
        KusaImage {
            dots: vec![Dot::default(); (width * height) as usize],
            width: width,
            height: height,
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
                    k_image.set_dot(col, row, &Dot::new(p[0], p[1], p[2], p[3]));
                    i += 1;
                }
                k_image
            }
            _ => KusaImage::new(1, 1),
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        for dot in &self.dots {
            vec.extend_from_slice(&dot.array());
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

    pub fn set_dot(&mut self, col: u32, row: u32, dot: &Dot) {
        println!(
            "Trace   | set_dot {} {} {} {}",
            col, row, self.width, self.height
        );
        self.dots[KusaImage::to_index(col, row, self.width, self.height)] = dot.clone();
    }

    pub fn get_dot(&self, col: u32, row: u32) -> &Dot {
        &self.dots[KusaImage::to_index(col, row, self.width, self.height)]
    }
}

#[derive(Clone)]
pub struct Dot {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Dot {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Dot {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }

    pub fn array(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub fn rate_array(&self) -> [f32; 4] {
        [
            self.r as f32 / 255f32,
            self.g as f32 / 255f32,
            self.b as f32 / 255f32,
            self.a as f32 / 255f32,
        ]
    }
}
impl Default for Dot {
    fn default() -> Self {
        Dot {
            r: 0,
            g: 128,
            b: 128,
            a: 255,
        }
    }
}
