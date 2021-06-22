use crate::data::input_state::InputState;
use crate::data::pointing::{Pointing, Sizing};
use crate::grid::Grid;
use crate::paint_tool::pen::Pen;
use crate::paint_tool::PaintOperation;
use crate::piston_wrapper::kusa_image::write_k_image;
use crate::piston_wrapper::kusa_image::KusaImage;
use crate::settings::Settings;
use piston_window::*;

pub fn show_window(mut settings: Settings, k_image: &mut KusaImage) {
    let opengl = OpenGL::V3_2;

    let width = settings.canvas_margin_left
        + settings.image_width as f64 * settings.canvas_dot_width
        + settings.canvas_margin_right;
    let height = settings.canvas_margin_top
        + settings.image_height as f64 * settings.canvas_dot_height
        + settings.canvas_margin_bottom;
    let mut window: PistonWindow = WindowSettings::new("piston: image", [width, height])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    // let texture = create_texture(&settings.image_file, &mut window);
    let mut input_state = InputState::default();
    let mut k_mouse_cursor = Pointing::default();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    //println!("{:?}", assets);
    let mut glyphs = window
        .load_font(assets.join("font/NotoSans-Medium.ttf"))
        .unwrap();

    let mut count_to_reload: u64 = 0;
    // Event loop.
    window.set_lazy(true);
    while let Some(e) = window.next() {
        if count_to_reload % 1000 == 999 {
            // ミリ秒の取り方が分からなかったぜ☆（＾～＾）
            // イベント・ループの中で　ファイル入出力するのは　クソだが　使い慣れてないんで仕方ないぜ☆（＾～＾）
            // 設定ファイルを監視するぜ☆（＾～＾）
            settings = Settings::load();
            //println!(
            //    "Trace   | Load settings☆（＾～＾） paint_tool=|{}|",
            //    settings.paint_tool
            //);
            count_to_reload = 0;
        } else {
            count_to_reload += 1;
        }
        // マウスカーソルの座標を補足するぜ☆（＾～＾）
        e.mouse_cursor(|pos| {
            k_mouse_cursor = Pointing::from_pos(pos);
        });

        // 📖 [Event](http://docs.piston.rs/piston_window/piston_window/enum.Event.html)
        // ⚡Mouse button pressed
        // 📖 [PressEvent](https://docs.piston.rs/piston_window/piston_window/trait.PressEvent.html)
        if let Some(_button) = e.press_args() {
            input_state.is_mouse_pressed = true;
            input_state.pressed_coord = k_mouse_cursor.clone();
            //println!("Trace   | ボタンが押されたぜ☆（＾～＾） {:?}", pressed_pos);

            match settings.paint_tool.as_str() {
                "pen" => {
                    // 点を置きます
                    Pen::put_dot(k_image, &input_state.pressed_coord, &settings);

                    // 保存
                    write_k_image(&k_image, &settings.image_file);
                }
                _ => {}
            }
        }

        // TODO ⚡Mouse move
        // 📖 [MouseRelativeEvent](https://docs.piston.rs/piston_window/piston_window/trait.MouseRelativeEvent.html)
        if let Some(coord) = e.mouse_relative_args() {
            if input_state.is_mouse_pressed {
                let dx = coord[0];
                let dy = coord[1];
                input_state.pressed_coord.x += dx;
                input_state.pressed_coord.y += dy;
                //println!(
                //    "Trace   | マウス移動中☆（＾～＾） ({:?}, {:?}) ({:?}, {:?})",
                //    dx, dy, pressed_pos.x, pressed_pos.y
                //);
            }
            match settings.paint_tool.as_str() {
                "pen" => {
                    if input_state.is_mouse_pressed {
                        // 点を置きます
                        Pen::put_dot(k_image, &input_state.pressed_coord, &settings);
                        // 保存
                        write_k_image(&k_image, &settings.image_file);
                    }
                }
                _ => {}
            }
        }

        // ⚡Mouse button released
        // 📖 [ReleaseEvent](https://docs.piston.rs/piston_window/piston_window/trait.ReleaseEvent.html)
        if let Some(_button) = e.release_args() {
            //println!("Trace   | ボタンを離したぜ☆（＾～＾）");
            input_state.is_mouse_pressed = false;
            match settings.paint_tool.as_str() {
                "pen" => {
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
                _ => {}
            }
        }

        // ⚡Window paint
        window.draw_2d(&e, |c, g, device| {
            clear([1.0; 4], g);

            /*
            // 画像を丸ごと再描画します。
            image(&texture, c.transform.zoom(settings.canvas_zoom), g);
            */

            // 点を１個描くぜ☆（＾～＾）データとしての保存は別のところでやってるぜ☆（＾～＾）
            // let sizing = Sizing::diff(&cursor, &pressed_pos);

            /*
            if sizing.is_longer_width() {
                // 横幅の方が長ければ。
                for col in 1..(sizing.long_len() + 1) {
                    let y = sizing.get_a() * col as f64;
                    rectangle(
                        [1.0, 0.0, 0.0, 1.0], // red
                        [
                            pressed_pos.x + sizing.width,
                            pressed_pos.y + y,
                            100.0,
                            100.0,
                        ],
                        c.transform,
                        g,
                    );
                }
            } else {
                // 縦幅の方が長いか同じなら。
                for row in 1..(sizing.long_len() + 1) {
                    let x = sizing.get_a() * row as f64;
                    rectangle(
                        [1.0, 0.0, 0.0, 1.0], // red
                        [
                            pressed_pos.x + x,
                            pressed_pos.y + sizing.height,
                            100.0,
                            100.0,
                        ],
                        c.transform,
                        g,
                    );
                }
            }
            */

            // 各マスに色を打っていくぜ☆（＾～＾）
            PaintOperation::draw_image(&settings, &k_image, &c, g);

            // TODO 今引こうとしている線を、データに描き込まずに画面に表示したいぜ☆（＾～＾）

            // グリッド
            Grid::draw(&settings, &Sizing::load_canvas(&settings), &c, g);

            // TODO 座標を表示したいぜ☆（＾～＾）
            text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32)
                .draw(
                    &format!("cell({:.0}, {:.0})", k_mouse_cursor.x, k_mouse_cursor.y),
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(10.0, 30.0), // y位置を揃えるのはむずかしいぜ☆（＾～＾）
                    g,
                )
                .unwrap();

            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);
        });
    }
}
