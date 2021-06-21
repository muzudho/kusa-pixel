use crate::settings::Settings;
use std::fmt;

#[derive(Clone, Copy)]
pub struct Pointing {
    // Screen coordinate
    pub x: f64,
    pub y: f64,
}
impl Default for Pointing {
    fn default() -> Self {
        Pointing { x: 0.0, y: 0.0 }
    }
}
impl Pointing {
    pub fn from_pos(pos: [f64; 2]) -> Self {
        Pointing {
            x: pos[0],
            y: pos[1],
        }
    }
}
impl fmt::Debug for Pointing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pos=({}, {})", self.x, self.y)
    }
}

#[derive(Clone, Copy)]
pub struct Sizing {
    pub width: f64,
    pub height: f64,
}
impl Sizing {
    pub fn load_canvas(settings: &Settings) -> Self {
        Sizing {
            width: settings.image_width as f64 * settings.canvas_dot_width,
            height: settings.image_height as f64 * settings.canvas_dot_height,
        }
    }
    pub fn diff(cursor: &Pointing, pressed_pos: &Pointing, settings: &Settings) -> Self {
        let sizing = Sizing {
            width: cursor.x - pressed_pos.x,
            height: cursor.y - pressed_pos.y,
        };
        //println!(
        //    "Trace   | diff cursor={:?} pressed_pos={:?} sizing={:?} long_edge_sign={} long_edge_cell_abs={}",
        //    cursor,
        //    pressed_pos,
        //    sizing,
        //    sizing.long_edge_sign(),
        //    sizing.long_edge_cells_abs()
        //);
        sizing
    }

    pub fn is_longer_edge_abs(&self) -> bool {
        self.height.abs() < self.width.abs()
    }

    pub fn get_a(&self) -> f64 {
        if self.is_longer_edge_abs() {
            self.height as f64 / self.width as f64
        } else {
            self.width as f64 / self.height as f64
        }
    }

    /*
    pub fn long_edge_pixels_abs(&self) -> usize {
        if self.is_longer_width_abs() {
            self.width.abs() as usize
        } else {
            self.height.abs() as usize
        }
    }
    */

    pub fn long_edge_cells_abs(&self, settings: &Settings) -> usize {
        if self.is_longer_edge_abs() {
            (self.width / settings.canvas_dot_width) as usize
        } else {
            (self.height / settings.canvas_dot_height) as usize
        }
    }

    /// 長い方の辺の正負を返します。 1 or -1.
    pub fn long_edge_sign(&self) -> f64 {
        if self.is_longer_edge_abs() {
            self.width / self.width.abs()
        } else {
            self.height / self.height.abs()
        }
    }
}
impl fmt::Debug for Sizing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pos=({}, {})", self.width, self.height)
    }
}
