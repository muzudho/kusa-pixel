use crate::res::image::{Dot, Frame};
use crate::res::pointing::{Pointing, Sizing};

pub struct Pen {}
impl Pen {
    pub fn draw(frame: &mut Frame, pressed_pos: &Pointing, sizing: &Sizing) {
        if sizing.is_longer_edge_abs() {
            // 横幅の方が長ければ。
            let horizontal = &mut |col| {
                let row = sizing.get_a() * col;
                // 点を１個打って画像として保存するぜ☆（＾～＾）画面への描画は別のところでやってるぜ☆（＾～＾）
                frame.set_dot(
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
                frame.set_dot(
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
