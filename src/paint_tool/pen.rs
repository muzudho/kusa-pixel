use crate::data::input_state::InputState;
use crate::data::pointing::KusaCell;
use crate::data::pointing::KusaPoint;
use crate::paint_tool::screen_to_image;
use crate::paint_tool::PaintTool;
use crate::piston_wrapper::kusa_image::KusaImage;
use crate::settings::Settings;
use crate::write_k_image;

pub struct Pen {}
impl PaintTool for Pen {
    fn on_mouse_pressed(
        &self,
        settings: &Settings,
        input_state: &InputState,
        k_image: &mut KusaImage,
    ) {
        // 点を置きます
        Pen::put_pixel(k_image, &input_state.pressed_point, &settings);

        // 保存
        write_k_image(&k_image, &settings.image_file);
    }
    fn on_mouse_moved(
        &self,
        settings: &Settings,
        input_state: &InputState,
        k_image: &mut KusaImage,
    ) {
        if input_state.is_mouse_pressed {
            // 移動した区間に連続した点を置きます
            self.draw_line(&settings, k_image, &input_state);
            //println!(
            //    "Trace   | Click ({}, {}) 保存",
            //    &k_mouse_cursor.x, &k_mouse_cursor.y
            //);
            write_k_image(&k_image, &settings.image_file);
            // 保存
            write_k_image(&k_image, &settings.image_file);
        }
    }
    fn on_mouse_released(
        &self,
        _settings: &Settings,
        _input_state: &InputState,
        _k_image: &mut KusaImage,
    ) {
    }
}
impl Pen {
    // 点を置くぜ（＾～＾）
    fn put_pixel(k_image: &mut KusaImage, sc_coord: &KusaPoint, settings: &Settings) {
        // 点を１個打って画像として保存するぜ☆（＾～＾）画面への描画は別のところでやってるぜ☆（＾～＾）
        if let Some(coord) = screen_to_image(sc_coord.x, sc_coord.y, settings) {
            k_image.set_pixel(coord.0 as u32, coord.1 as u32, &settings.paint_color);
        }
    }

    // 線を引くぜ（＾～＾）
    fn draw_line(&self, settings: &Settings, k_image: &mut KusaImage, input_state: &InputState) {
        // 画像上のピクセル位置
        if let Some(previous_cell) = screen_to_image(
            input_state.previous_point.x,
            input_state.previous_point.y,
            settings,
        ) {
            println!(
                "Trace   | previous_cell=({} {})",
                previous_cell.0, previous_cell.1
            );

            // 横長
            let landscape = input_state.moved_vector.y.abs() < input_state.moved_vector.x.abs();
            println!("Trace   | landscape={}", landscape);

            let end_cell = KusaCell {
                x: ((input_state.previous_point.x + input_state.moved_vector.x)
                    / settings.canvas_dot_width) as i32,
                y: ((input_state.previous_point.y + input_state.moved_vector.y)
                    / settings.canvas_dot_height) as i32,
            };

            println!("Trace   | end_cell=({} {})", end_cell.x, end_cell.y);

            // 画像上のピクセル数を返します
            let dx = end_cell.x - previous_cell.0;
            let dy = end_cell.y - previous_cell.1;
            // ずっと |1|未満 だと何も描かれないので、
            // 1未満は 1に切り上げます。-1未満は-1に切り上げます
            let dx_len = dx.abs();
            let dy_len = dy.abs();

            // 長い方の辺の正負を返します。 1 or -1
            let long_edge_sign = if landscape {
                if 0 <= dx {
                    1
                } else {
                    -1
                }
            } else {
                if 0 <= dy {
                    1
                } else {
                    -1
                }
            };
            // 短い方の辺の比を返します
            let short_edge_rate = if landscape {
                if 0 < dx_len {
                    dy_len as f64 / dx_len as f64
                } else {
                    0.0
                }
            } else {
                if 0 < dy_len {
                    dx_len as f64 / dy_len as f64
                } else {
                    0.0
                }
            };
            if landscape {
                // 横幅の方が長ければ。
                let draw_horizontal = &mut |interpolation_x| {
                    let interpolation_y = (short_edge_rate * interpolation_x as f64) as i32;
                    // 点を１個打って画像として保存するぜ☆（＾～＾）画面への描画は別のところでやってるぜ☆（＾～＾）
                    let im_x = previous_cell.0 + interpolation_x;
                    let im_y = previous_cell.1 + interpolation_y;
                    //println!("Trace   | 右へ（＾～＾） im_x={} im_y={}", im_x, im_y);
                    if 0 <= im_x
                        && im_x < settings.image_width as i32
                        && 0 <= im_y
                        && im_y < settings.image_height as i32
                    {
                        k_image.set_pixel(im_x as u32, im_y as u32, &settings.paint_color);
                    }
                };
                if 0 <= long_edge_sign {
                    println!("Trace   | 右へ☆（＾～＾） dx_len={}", dx_len);
                    for x in 1..(dx_len as i32 + 1) {
                        draw_horizontal(x);
                    }
                } else {
                    println!("Trace   | 左へ☆（＾～＾）");
                    for x in (1..(dx_len as i32 + 1)).rev() {
                        draw_horizontal(long_edge_sign * x);
                    }
                }
            } else {
                // 縦幅の方が長いか同じなら。
                let draw_vertical = &mut |interpolation_y| {
                    println!(
                        "Trace   | short_edge_rate={} interpolation_y={}",
                        short_edge_rate, interpolation_y
                    );
                    let interpolation_x = (short_edge_rate * interpolation_y as f64) as i32;
                    println!("Trace   | interpolation_x={}", interpolation_x,);
                    // 点を１個打って画像として保存するぜ☆（＾～＾）画面への描画は別のところでやってるぜ☆（＾～＾）
                    let im_x = previous_cell.0 + interpolation_x;
                    let im_y = previous_cell.1 + interpolation_y;
                    println!("Trace   | im_x={} im_y={}", im_x, im_y);
                    if 0 <= im_x
                        && im_x < settings.image_width as i32
                        && 0 <= im_y
                        && im_y < settings.image_height as i32
                    {
                        k_image.set_pixel(im_x as u32, im_y as u32, &settings.paint_color);
                    }
                };
                if 0 <= long_edge_sign {
                    println!("Trace   | 下へ☆（＾～＾）");
                    for y in 1..(dy_len as i32 + 1) {
                        draw_vertical(y);
                    }
                } else {
                    println!("Trace   | 上へ☆（＾～＾）");
                    for y in (1..(dy_len as i32 + 1)).rev() {
                        draw_vertical(long_edge_sign * y);
                    }
                }
            }
        } else {
            // 画像の外をクリックしていても無視します
        }
    }
}
