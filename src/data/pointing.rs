use std::fmt;

#[derive(Clone, Copy)]
pub struct KusaPoint {
    pub x: f64,
    pub y: f64,
}
impl Default for KusaPoint {
    fn default() -> Self {
        KusaPoint { x: 0.0, y: 0.0 }
    }
}
impl KusaPoint {
    pub fn from_pos(pos: [f64; 2]) -> Self {
        KusaPoint {
            x: pos[0],
            y: pos[1],
        }
    }
}
impl fmt::Debug for KusaPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "point=({}, {})", self.x, self.y)
    }
}

#[derive(Clone, Copy)]
pub struct KusaSize {
    pub width: f64,
    pub height: f64,
}
impl KusaSize {
    /*
    pub fn diff(settings: &Settings, moved_point: &KusaPoint, pressed_coord: &KusaPoint) -> Self {
        let sizing = KusaSize {
            width: moved_point.x - pressed_coord.x,
            height: moved_point.y - pressed_coord.y,
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
    */

    /*
    pub fn long_edge_pixels_abs(&self) -> usize {
        if self.is_longer_width_abs() {
            self.width.abs() as usize
        } else {
            self.height.abs() as usize
        }
    }
    */
}
impl fmt::Debug for KusaSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pos=({}, {})", self.width, self.height)
    }
}
