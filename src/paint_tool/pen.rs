use crate::data::pointing::Pointing;
use crate::paint_tool::coord_on_image;
use crate::piston_wrapper::kusa_image::KusaImage;
use crate::settings::Settings;

pub struct Pen {}
impl Pen {
    // 点を置くぜ（＾～＾）
    pub fn put_dot(k_image: &mut KusaImage, pointing: &Pointing, settings: &Settings) {
        // 点を１個打って画像として保存するぜ☆（＾～＾）画面への描画は別のところでやってるぜ☆（＾～＾）
        if let Some(coord) = coord_on_image(pointing.x, pointing.y, settings) {
            k_image.set_pixel(coord.0 as u32, coord.1 as u32, &settings.paint_color);
        }
    }

    /*
    // 線を引くぜ（＾～＾）
    pub fn draw_line(
        k_image: &mut KusaImage,
        pressed_pos: &Pointing,
        sizing: &Sizing,
        settings: &Settings,
    ) {
        if sizing.is_longer_edge_abs() {
            // 横幅の方が長ければ。
            let horizontal = &mut |col| {
                let row = sizing.get_a() * col;
                // 点を１個打って画像として保存するぜ☆（＾～＾）画面への描画は別のところでやってるぜ☆（＾～＾）
                if let Some(coord) = coord_on_image(pressed_pos.x, pressed_pos.y, settings) {
                    k_image.set_dot(
                        (coord.0 + col as i32) as u32,
                        (coord.1 + row as i32) as u32,
                        &settings.paint_color,
                    );
                }
            };
            if 0.0 <= sizing.long_edge_sign() {
                //println!("Trace   | 左へ☆（＾～＾）");
                for col in 1..(sizing.long_edge_cells_abs(settings) + 1) {
                    horizontal(col as f64);
                }
            } else {
                //println!("Trace   | 右へ☆（＾～＾）");
                for col in (1..(sizing.long_edge_cells_abs(settings) + 1)).rev() {
                    horizontal(sizing.long_edge_sign() * (col as f64));
                }
            }
        } else {
            // 縦幅の方が長いか同じなら。
            let vertical = &mut |row| {
                let col = sizing.get_a() * row;
                //println!("Trace   | col {} = {} * {}", col, sizing.get_a(), row);
                // 点を１個打って画像として保存するぜ☆（＾～＾）画面への描画は別のところでやってるぜ☆（＾～＾）
                if let Some(coord) = coord_on_image(pressed_pos.x, pressed_pos.y, settings) {
                    k_image.set_dot(
                        (coord.0 + col as i32) as u32,
                        (coord.1 + row as i32) as u32,
                        &settings.paint_color,
                    );
                }
            };
            if 0.0 <= sizing.long_edge_sign() {
                //println!("Trace   | 下へ☆（＾～＾）");
                for row in 1..(sizing.long_edge_cells_abs(settings) + 1) {
                    vertical(row as f64);
                }
            } else {
                //println!("Trace   | 上へ☆（＾～＾）");
                for row in (1..(sizing.long_edge_cells_abs(settings) + 1)).rev() {
                    vertical(sizing.long_edge_sign() * (row as f64));
                }
            }
        }
    }
    */
}
