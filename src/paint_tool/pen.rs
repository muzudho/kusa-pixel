use crate::data::input_state::InputState;
use crate::data::pointing::Pointing;
use crate::paint_tool::coord_on_image;
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
        Pen::put_pixel(k_image, &input_state.pressed_coord, &settings);

        // 保存
        write_k_image(&k_image, &settings.image_file);
    }
    fn on_mouse_moved(
        &self,
        settings: &Settings,
        input_state: &InputState,
        k_image: &mut KusaImage,
        dx: f64,
        dy: f64,
    ) {
        if input_state.is_mouse_pressed {
            // 移動した区間に連続した点を置きます
            Pen::draw_line(&settings, k_image, &input_state.pressed_coord, dx, dy);
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
    fn put_pixel(k_image: &mut KusaImage, pointing: &Pointing, settings: &Settings) {
        // 点を１個打って画像として保存するぜ☆（＾～＾）画面への描画は別のところでやってるぜ☆（＾～＾）
        if let Some(coord) = coord_on_image(pointing.x, pointing.y, settings) {
            k_image.set_pixel(coord.0 as u32, coord.1 as u32, &settings.paint_color);
        }
    }

    // 線を引くぜ（＾～＾）
    //
    // # Arguments
    //
    // * `sc_dx` - スクリーン上の差分x
    // * `sc_dy` - スクリーン上の差分y
    fn draw_line(
        settings: &Settings,
        k_image: &mut KusaImage,
        pressed_sc_coord: &Pointing,
        sc_dx: f64,
        sc_dy: f64,
    ) {
        if let Some(pressed_im_coord) =
            coord_on_image(pressed_sc_coord.x, pressed_sc_coord.y, settings)
        {
            println!(
                "Trace   | pressed_im_coord.0={} pressed_im_coord.1={}",
                pressed_im_coord.0, pressed_im_coord.1
            );

            // 画像上のピクセル数を返します
            let im_d_width = sc_dx / settings.canvas_dot_width;
            let im_width = im_d_width.abs();
            let im_d_height = sc_dy / settings.canvas_dot_height;
            let im_height = im_d_height.abs();
            // 横長
            let im_landscape = im_height < im_width;
            // 長い方の辺の正負を返します。 1 or -1
            let im_long_edge_sign = if im_landscape {
                if im_d_width.is_sign_positive() {
                    1
                } else {
                    -1
                }
            } else {
                if im_d_height.is_sign_positive() {
                    1
                } else {
                    -1
                }
            };
            // 短い方の辺の比を返します
            let im_short_edge_rate = if im_landscape {
                if 0.0 < im_width {
                    im_height / im_width
                } else {
                    0.0
                }
            } else {
                if 0.0 < im_height {
                    im_width / im_height
                } else {
                    0.0
                }
            };
            if im_landscape {
                // 横幅の方が長ければ。
                let draw_horizontal = &mut |im_d_col| {
                    let im_d_row = (im_short_edge_rate * im_d_col as f64) as i32;
                    // 点を１個打って画像として保存するぜ☆（＾～＾）画面への描画は別のところでやってるぜ☆（＾～＾）
                    let im_x = pressed_im_coord.0 + im_d_col;
                    let im_y = pressed_im_coord.1 + im_d_row;
                    if 0 <= im_x
                        && im_x < settings.image_width as i32
                        && 0 <= im_y
                        && im_y < settings.image_height as i32
                    {
                        k_image.set_pixel(im_x as u32, im_y as u32, &settings.paint_color);
                    }
                };
                if 0 <= im_long_edge_sign {
                    //println!("Trace   | 左へ☆（＾～＾）");
                    for im_d_col in 1..(im_width as i32 + 1) {
                        draw_horizontal(im_d_col);
                    }
                } else {
                    //println!("Trace   | 右へ☆（＾～＾）");
                    for im_d_col in (1..(im_width as i32 + 1)).rev() {
                        draw_horizontal(im_long_edge_sign * im_d_col);
                    }
                }
            } else {
                // 縦幅の方が長いか同じなら。
                let draw_vertical = &mut |im_d_row| {
                    println!(
                        "Trace   | im_short_edge_rate={} im_d_row={}",
                        im_short_edge_rate, im_d_row
                    );
                    let im_d_col = (im_short_edge_rate * im_d_row as f64) as i32;
                    println!("Trace   | im_d_col={}", im_d_col,);
                    // 点を１個打って画像として保存するぜ☆（＾～＾）画面への描画は別のところでやってるぜ☆（＾～＾）
                    let im_x = pressed_im_coord.0 + im_d_col;
                    let im_y = pressed_im_coord.1 + im_d_row;
                    println!("Trace   | im_x={} im_y={}", im_x, im_y);
                    if 0 <= im_x
                        && im_x < settings.image_width as i32
                        && 0 <= im_y
                        && im_y < settings.image_height as i32
                    {
                        k_image.set_pixel(im_x as u32, im_y as u32, &settings.paint_color);
                    }
                };
                if 0 <= im_long_edge_sign {
                    //println!("Trace   | 下へ☆（＾～＾）");
                    for im_d_row in 1..(im_height as i32 + 1) {
                        draw_vertical(im_d_row);
                    }
                } else {
                    //println!("Trace   | 上へ☆（＾～＾）");
                    for im_d_row in (1..(im_height as i32 + 1)).rev() {
                        draw_vertical(im_long_edge_sign * im_d_row);
                    }
                }
            }
        } else {
            // 画像の外をクリックしていても無視します
        }
    }
}
