use crate::settings::Settings;
use std::fmt;

#[derive(Clone, Copy)]
pub struct Pointing {
    pub x: f64,
    pub y: f64,
    pub col: i32,
    pub row: i32,
}
impl Default for Pointing {
    fn default() -> Self {
        Pointing {
            x: 0.0,
            y: 0.0,
            col: 0,
            row: 0,
        }
    }
}
impl Pointing {
    pub fn from_pos(pos: [f64; 2], settings: &Settings) -> Self {
        Pointing {
            x: pos[0],
            y: pos[1],
            col: ((pos[0] - settings.canvas_margin_x) / settings.canvas_dot_width) as i32,
            row: ((pos[1] - settings.canvas_margin_y) / settings.canvas_dot_height) as i32,
        }
    }
}
impl fmt::Debug for Pointing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "pos=({}, {}) cell=({}, {})",
            self.x, self.y, self.col, self.row
        )
    }
}

#[derive(Clone, Copy)]
pub struct Sizing {
    pub width: f64,
    pub height: f64,
    pub cols: i32,
    pub rows: i32,
}
impl Sizing {
    pub fn load_canvas(settings: &Settings) -> Self {
        Sizing {
            width: settings.image_width as f64 * settings.canvas_dot_width,
            height: settings.image_height as f64 * settings.canvas_dot_height,
            cols: settings.image_width as i32,
            rows: settings.image_height as i32,
        }
    }
    pub fn diff(cursor: &Pointing, pressed_pos: &Pointing) -> Self {
        let sizing = Sizing {
            width: cursor.x - pressed_pos.x,
            height: cursor.y - pressed_pos.y,
            cols: cursor.col - pressed_pos.col,
            rows: cursor.row - pressed_pos.row,
        };
        println!(
            "Trace   | diff cursor={:?} pressed_pos={:?} sizing={:?} long_edge_sign={} long_edge_cell_abs={}",
            cursor,
            pressed_pos,
            sizing,
            sizing.long_edge_sign(),
            sizing.long_edge_cells_abs()
        );
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

    pub fn long_edge_cells_abs(&self) -> usize {
        if self.is_longer_edge_abs() {
            self.cols.abs() as usize
        } else {
            self.rows.abs() as usize
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
        write!(
            f,
            "pos=({}, {}) cell=({}, {})",
            self.width, self.height, self.cols, self.rows
        )
    }
}
