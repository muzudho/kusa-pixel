pub struct Frame {
    pub dots: Vec<Dot>,
    pub width: u32,
    pub height: u32,
}
impl Frame {
    pub fn new(width: u32, height: u32) -> Self {
        Frame {
            dots: vec![Dot::default(); (width * height) as usize],
            width: width,
            height: height,
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        for dot in &self.dots {
            vec.extend_from_slice(&dot.array());
        }
        vec
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
    pub fn array(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
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
