use crate::res::settings::Settings;
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
    pub fn diff(cursor: &Pointing, pressed_pos: &Pointing) -> Self {
        Sizing {
            width: cursor.x - pressed_pos.x,
            height: cursor.y - pressed_pos.y,
            cols: cursor.col - pressed_pos.col,
            rows: cursor.row - pressed_pos.row,
        }
    }

    pub fn is_longer_width(&self) -> bool {
        self.height < self.width
    }

    pub fn get_a(&self) -> f64 {
        if self.is_longer_width() {
            self.height as f64 / self.width as f64
        } else {
            self.width as f64 / self.height as f64
        }
    }

    pub fn long_len(&self) -> usize {
        if self.is_longer_width() {
            self.width as usize
        } else {
            self.height as usize
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
