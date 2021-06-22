use crate::data::input_state::InputState;
use crate::data::pointing::KusaPoint;
use crate::paint_tool::screen_to_image;
use crate::paint_tool::square_pen_nibs::SquarePenNibs;
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
        // PixelPenNibs::put_pixel(&settings, k_image, &input_state.pressed_point);
        SquarePenNibs::put_pixel(&settings, k_image, &input_state.pressed_point);

        // 保存
        write_k_image(&k_image, &settings.image_file);
    }
    fn on_mouse_moved(
        &self,
        settings: &Settings,
        input_state: &InputState,
        k_image: &mut KusaImage,
    ) -> bool {
        if input_state.is_mouse_pressed {
            // 移動した区間に連続した点を置きます
            if !self.draw_line(&settings, k_image, &input_state) {
                return false;
            }
            //println!(
            //    "Trace   | Click ({}, {}) 保存",
            //    &k_mouse_cursor.x, &k_mouse_cursor.y
            //);
            write_k_image(&k_image, &settings.image_file);
            // 保存
            write_k_image(&k_image, &settings.image_file);
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
        k_image: &mut KusaImage,
        input_state: &InputState,
    ) -> bool {
        // 画像上のピクセル位置
        if let Some(previous_cell) = screen_to_image(settings, &input_state.previous_point) {
            // println!(
            //     "Trace   | previous_cell=({} {})",
            //     previous_cell.0, previous_cell.1
            // );

            if let Some(end_cell) = screen_to_image(
                settings,
                &KusaPoint {
                    x: input_state.previous_point.x + input_state.moved_vector.x,
                    y: input_state.previous_point.y + input_state.moved_vector.y,
                },
            ) {
                // 横長
                let landscape = input_state.moved_vector.y.abs() < input_state.moved_vector.x.abs();
                //println!("Trace   | landscape={}", landscape);
                //println!("Trace   | end_cell=({} {})", end_cell.x, end_cell.y);

                // TODO previous と end が同じなら何もしません
                if end_cell.x == previous_cell.x && end_cell.y == previous_cell.y {
                    return false;
                }

                // 画像上のピクセル数を返します
                let dx = end_cell.x - previous_cell.x;
                let dy = end_cell.y - previous_cell.y;
                // println!(
                //     "Trace   | dx={} end_cell.x={} previous_cell.0={}",
                //     dx, end_cell.x, previous_cell.x
                // );

                // ずっと |1|未満 だと何も描かれないので、
                // 1未満は 1に切り上げます。-1未満は-1に切り上げます
                let dx_len = dx.abs();
                let dy_len = dy.abs();

                // 長い方の辺の長さ
                let long_edge_len = if landscape { dx_len } else { dy_len };

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
                        let im_x = previous_cell.x + interpolation_x;
                        let im_y = previous_cell.y + interpolation_y;
                        //println!("Trace   | im_x={} im_y={}", im_x, im_y);
                        if 0 <= im_x
                            && im_x < settings.image_width as i32
                            && 0 <= im_y
                            && im_y < settings.image_height as i32
                        {
                            k_image.set_pixel(im_x as u32, im_y as u32, &settings.paint_color);
                        }
                    };
                    if 0 <= long_edge_sign {
                        //println!("Trace   | 右へ☆（＾～＾） dx_len={}", dx_len);
                        for x in 0..long_edge_len {
                            draw_horizontal(x);
                        }
                    } else {
                        //println!("Trace   | 左へ☆（＾～＾） ");
                        for x in 0..long_edge_len {
                            draw_horizontal(long_edge_sign * x);
                        }
                    }
                } else {
                    // 縦幅の方が長いか同じなら。
                    let draw_vertical = &mut |interpolation_y| {
                        // println!(
                        //     "Trace   | short_edge_rate={} interpolation_y={}",
                        //     short_edge_rate, interpolation_y
                        // );
                        let interpolation_x = (short_edge_rate * interpolation_y as f64) as i32;
                        //println!("Trace   | interpolation_x={}", interpolation_x,);
                        // 点を１個打って画像として保存するぜ☆（＾～＾）画面への描画は別のところでやってるぜ☆（＾～＾）
                        let im_x = previous_cell.x + interpolation_x;
                        let im_y = previous_cell.y + interpolation_y;
                        //println!("Trace   | im_x={} im_y={}", im_x, im_y);
                        if 0 <= im_x
                            && im_x < settings.image_width as i32
                            && 0 <= im_y
                            && im_y < settings.image_height as i32
                        {
                            k_image.set_pixel(im_x as u32, im_y as u32, &settings.paint_color);
                        }
                    };
                    if 0 <= long_edge_sign {
                        //println!("Trace   | 下へ☆（＾～＾）");
                        for y in 0..long_edge_len {
                            draw_vertical(y);
                        }
                    } else {
                        //println!("Trace   | 上へ☆（＾～＾）");
                        for y in 0..long_edge_len {
                            draw_vertical(long_edge_sign * y);
                        }
                    }
                }

                // 終点を塗ります
                k_image.set_pixel(end_cell.x as u32, end_cell.y as u32, &settings.paint_color);

                return true;
            }
            // 画像の外をクリックしていても無視します
            return false;
        }
        // 画像の外をクリックしていても無視します
        return false;
    }
}
