use crate::data::input_state::InputState;
use crate::data::pointing::KusaPoint;
use crate::paint_tool::screen_to_image;
use crate::paint_tool::screen_to_image_f;
use crate::paint_tool::Nib;
use crate::paint_tool::PaintTool;
use crate::piston_wrapper::kusa_image::KusaImage;
use crate::settings::Settings;

pub struct Pen {}
impl PaintTool for Pen {
    fn on_mouse_pressed(
        &self,
        settings: &Settings,
        nib: &dyn Nib,
        input_state: &InputState,
        k_image: &mut KusaImage,
    ) {
        if let Some(center) = screen_to_image_f(settings, &input_state.pressed_point) {
            // 点を置きます
            nib.put_pixel(&settings, k_image, &center);

            k_image.dirty = true;
        }
    }
    fn on_mouse_moved(
        &self,
        settings: &Settings,
        nib: &dyn Nib,
        input_state: &InputState,
        k_image: &mut KusaImage,
    ) -> bool {
        if input_state.is_mouse_pressed {
            // 移動した区間に連続した点を置きます
            if !self.draw_line(&settings, nib, k_image, &input_state) {
                return false;
            }
            //println!(
            //    "Trace   | Click ({}, {}) 保存",
            //    &k_mouse_cursor.x, &k_mouse_cursor.y
            //);
            k_image.dirty = true;
            return true;
        }
        return false;
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
    /*
    /// TODO 円を塗り潰すぜ（＾～＾）
    ///
    /// # Arguments
    ///
    /// * `sc_center` - スクリーン座標で円の中心位置
    fn fill_circle(settings: &Settings, k_image: &mut KusaImage, sc_center: &KusaPoint) {
        if let Some(coord) = screen_to_image(settings, sc_center) {
            k_image.set_pixel(coord.x as u32, coord.y as u32, &settings.paint_color);
        }
    }
    */

    // 線を引くぜ（＾～＾）
    fn draw_line(
        &self,
        settings: &Settings,
        nib: &dyn Nib,
        k_image: &mut KusaImage,
        input_state: &InputState,
    ) -> bool {
        // 画像上のピクセル位置
        if let Some(previous_cell) = screen_to_image(settings, &input_state.previous_point) {
            // println!(
            //     "Trace   | previous_cell=({} {})",
            //     previous_cell.0, previous_cell.1
            // );

            let end_point_x = input_state.previous_point.x + input_state.moved_vector.x;
            let end_point_y = input_state.previous_point.y + input_state.moved_vector.y;

            if let Some(end_cell) = screen_to_image(
                settings,
                &KusaPoint {
                    x: end_point_x,
                    y: end_point_y,
                },
            ) {
                // 横長
                let landscape = input_state.moved_vector.y.abs() < input_state.moved_vector.x.abs();
                //println!("Trace   | landscape={}", landscape);
                //println!("Trace   | end_cell=({} {})", end_cell.x, end_cell.y);

                /*
                // TODO previous と end が同じなら何もしません
                if end_cell.x == previous_cell.x && end_cell.y == previous_cell.y {
                    return false;
                }
                */

                // スクリーン上の長さを返します
                let horizontal_len = end_point_x - input_state.previous_point.x;
                let vertical_len = end_point_y - input_state.previous_point.y;
                // 短い方の辺の比を返します
                let shorter_side_rate = if landscape {
                    if 0.0 < horizontal_len.abs() {
                        vertical_len.abs() / horizontal_len.abs()
                    } else {
                        0.0
                    }
                } else {
                    if 0.0 < vertical_len.abs() {
                        horizontal_len.abs() / vertical_len.abs()
                    } else {
                        0.0
                    }
                };
                // 長い方の辺の正負を返します。 1 or -1
                let longer_side_sign = if landscape {
                    if 0.0 <= horizontal_len {
                        1
                    } else {
                        -1
                    }
                } else {
                    if 0.0 <= vertical_len {
                        1
                    } else {
                        -1
                    }
                };

                // 画像上のピクセル数を返します
                let d_columns = end_cell.x - previous_cell.x;
                let d_rows = end_cell.y - previous_cell.y;
                // println!(
                //     "Trace   | dx={} end_cell.x={} previous_cell.0={}",
                //     dx, end_cell.x, previous_cell.x
                // );

                // ずっと |1|未満 だと何も描かれないので、
                // 1未満は 1に切り上げます。-1未満は-1に切り上げます
                let d_columns_len = d_columns.abs();
                let d_rows_len = d_rows.abs();

                // 長い方の辺の長さ
                let longer_side_cells = if landscape { d_columns_len } else { d_rows_len };

                if landscape {
                    // 横幅の方が長ければ。
                    let draw_horizontal = &mut |interpolation_x: f64| {
                        let interpolation_y = shorter_side_rate * interpolation_x;
                        // 点を１個打って画像として保存するぜ☆（＾～＾）画面への描画は別のところでやってるぜ☆（＾～＾）
                        let x = input_state.previous_point.x as f64 + interpolation_x;
                        let y = input_state.previous_point.y as f64 + interpolation_y;
                        if 0.0 <= x
                            && x < settings.image_width as f64
                            && 0.0 <= y
                            && y < settings.image_height as f64
                        {
                            // println!("Trace   | 水平移動 x={} y={}", x, y);
                            // 点を置きます
                            nib.put_pixel(&settings, k_image, &KusaPoint { x: x, y: y });
                            //k_image.set_pixel(im_x as u32, im_y as u32, &settings.paint_color);
                        }
                    };
                    if 0 <= longer_side_sign {
                        // println!("Trace   | 右へ☆（＾～＾） dx_len={}", dx_len);
                        for x in 0..longer_side_cells {
                            draw_horizontal(x as f64);
                        }
                    } else {
                        //println!("Trace   | 左へ☆（＾～＾） ");
                        for x in 0..longer_side_cells {
                            draw_horizontal(longer_side_sign as f64 * x as f64);
                        }
                    }
                } else {
                    // 縦幅の方が長いか同じなら。
                    let draw_vertical = &mut |interpolation_y: f64| {
                        // println!(
                        //     "Trace   | shorter_side_rate={} interpolation_y={}",
                        //     shorter_side_rate, interpolation_y
                        // );
                        let interpolation_x = shorter_side_rate * interpolation_y;
                        //println!("Trace   | interpolation_x={}", interpolation_x,);
                        // 点を１個打って画像として保存するぜ☆（＾～＾）画面への描画は別のところでやってるぜ☆（＾～＾）
                        let x = input_state.previous_point.x as f64 + interpolation_x;
                        let y = input_state.previous_point.y as f64 + interpolation_y;
                        //println!("Trace   | im_x={} im_y={}", im_x, im_y);
                        if 0.0 <= x
                            && x < settings.image_width as f64
                            && 0.0 <= y
                            && y < settings.image_height as f64
                        {
                            // 点を置きます
                            nib.put_pixel(&settings, k_image, &KusaPoint { x: x, y: y });
                            // k_image.set_pixel(im_x as u32, im_y as u32, &settings.paint_color);
                        }
                    };
                    if 0 <= longer_side_sign {
                        //println!("Trace   | 下へ☆（＾～＾）");
                        for y in 0..longer_side_cells {
                            draw_vertical(y as f64);
                        }
                    } else {
                        //println!("Trace   | 上へ☆（＾～＾）");
                        for y in 0..longer_side_cells {
                            draw_vertical(longer_side_sign as f64 * y as f64);
                        }
                    }
                }

                // 終点を塗ります
                nib.put_pixel(
                    &settings,
                    k_image,
                    &KusaPoint {
                        x: end_cell.x as f64,
                        y: end_cell.y as f64,
                    },
                );

                return true;
            }
            // 画像の外をクリックしていても無視します
            return false;
        }
        // 画像の外をクリックしていても無視します
        return false;
    }
}
