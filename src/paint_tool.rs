use crate::data::pointing::{Pointing, Sizing};
use crate::piston_wrapper::kusa_image::{Dot, KusaImage};
use crate::settings::Settings;
use piston_window::*;

pub struct PaintOperation {}
impl PaintOperation {
    /// 各マスに色を打っていくぜ☆（＾～＾）
    pub fn draw(settings: &Settings, k_image: &KusaImage, c: &Context, g: &mut G2d) {
        // タテへ
        for row in 0..settings.image_height {
            // ヨコへ
            for col in 0..settings.image_width {
                let dot = k_image.get_dot(col, row);
                let x = col as f64 * settings.canvas_dot_width + settings.canvas_margin_x;
                let y = row as f64 * settings.canvas_dot_height + settings.canvas_margin_y;
                rectangle(
                    dot.rate_array(),
                    [x, y, settings.canvas_dot_width, settings.canvas_dot_height],
                    c.transform,
                    g,
                );
            }
        }
    }
}

pub struct Pen {}
impl Pen {
    pub fn draw_line(k_image: &mut KusaImage, pressed_pos: &Pointing, sizing: &Sizing) {
        if sizing.is_longer_edge_abs() {
            // 横幅の方が長ければ。
            let horizontal = &mut |col| {
                let row = sizing.get_a() * col;
                // 点を１個打って画像として保存するぜ☆（＾～＾）画面への描画は別のところでやってるぜ☆（＾～＾）
                k_image.set_dot(
                    (pressed_pos.col + col as i32) as u32,
                    (pressed_pos.row + row as i32) as u32,
                    &Dot::new(255, 0, 0, 255),
                );
            };
            if 0.0 <= sizing.long_edge_sign() {
                println!("Trace   | 左へ☆（＾～＾）");
                for col in 1..(sizing.long_edge_cells_abs() + 1) {
                    horizontal(col as f64);
                }
            } else {
                println!("Trace   | 右へ☆（＾～＾）");
                for col in (1..(sizing.long_edge_cells_abs() + 1)).rev() {
                    horizontal(sizing.long_edge_sign() * (col as f64));
                }
            }
        } else {
            // 縦幅の方が長いか同じなら。
            let vertical = &mut |row| {
                let col = sizing.get_a() * row;
                println!("Trace   | col {} = {} * {}", col, sizing.get_a(), row);
                // 点を１個打って画像として保存するぜ☆（＾～＾）画面への描画は別のところでやってるぜ☆（＾～＾）
                k_image.set_dot(
                    (pressed_pos.col + col as i32) as u32,
                    (pressed_pos.row + row as i32) as u32,
                    &Dot::new(255, 0, 0, 255),
                );
            };
            if 0.0 <= sizing.long_edge_sign() {
                println!("Trace   | 下へ☆（＾～＾）");
                for row in 1..(sizing.long_edge_cells_abs() + 1) {
                    vertical(row as f64);
                }
            } else {
                println!("Trace   | 上へ☆（＾～＾）");
                for row in (1..(sizing.long_edge_cells_abs() + 1)).rev() {
                    vertical(sizing.long_edge_sign() * (row as f64));
                }
            }
        }
    }
}
