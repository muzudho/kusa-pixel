use crate::data::input_state::InputState;
use crate::data::pointing::Pointing;
use crate::paint_tool::coord_on_image;
use crate::piston_wrapper::kusa_image::KusaImage;
use crate::settings::Settings;
use crate::write_k_image;

pub struct Pen {}
impl Pen {
    pub fn on_mouse_pressed(
        settings: &Settings,
        input_state: &InputState,
        k_image: &mut KusaImage,
    ) {
        // 点を置きます
        Pen::put_dot(k_image, &input_state.pressed_coord, &settings);

        // 保存
        write_k_image(&k_image, &settings.image_file);
    }
    pub fn on_mouse_moved(settings: &Settings, input_state: &InputState, k_image: &mut KusaImage) {
        if input_state.is_mouse_pressed {
            // 点を置きます
            Pen::put_dot(k_image, &input_state.pressed_coord, &settings);
            // 保存
            write_k_image(&k_image, &settings.image_file);
        }
    }
    pub fn on_mouse_released(
        _settings: &Settings,
        _input_state: &InputState,
        _k_image: &mut KusaImage,
    ) {
        /*
        let sizing = Sizing::diff(&k_mouse_cursor, &pressed_pos);

        // 線を引きます。
        Pen::draw_line(k_image, &pressed_pos, &sizing);

        //println!(
        //    "Trace   | Click ({}, {}) 保存",
        //    &k_mouse_cursor.x, &k_mouse_cursor.y
        //);
        write_k_image(&k_image, &settings.image_file);
        */
    }
    // 点を置くぜ（＾～＾）
    fn put_dot(k_image: &mut KusaImage, pointing: &Pointing, settings: &Settings) {
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
