use crate::data::input_state::InputState;
use crate::data::pointing::{KusaPoint, KusaSize};
use crate::grid::Grid;
use crate::paint_tool::pen::*;
use crate::paint_tool::PaintOperation;
use crate::paint_tool::PaintTool;
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
    let mut k_mouse_cursor = KusaPoint::default();
    let mut paint_tool = match settings.paint_tool.as_str() {
        "pen" => Pen {},
        _ => Pen {},
    };

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

            paint_tool = match settings.paint_tool.as_str() {
                "pen" => Pen {},
                _ => Pen {},
            };
            //println!(
            //    "Trace   | Load settings☆（＾～＾） paint_tool=|{}|",
            //    settings.paint_tool
            //);
            count_to_reload = 0;
        } else {
            count_to_reload += 1;
        }
        // マウスカーソルの座標を補足するぜ☆（＾～＾）
        e.mouse_cursor(|coord| {
            k_mouse_cursor = KusaPoint::from_coord(coord);
        });

        // 📖 [Event](http://docs.piston.rs/piston_window/piston_window/enum.Event.html)
        // ⚡Mouse button pressed
        // 📖 [PressEvent](https://docs.piston.rs/piston_window/piston_window/trait.PressEvent.html)
        if let Some(_button) = e.press_args() {
            input_state.is_mouse_pressed = true;
            input_state.pressed_point = k_mouse_cursor.clone();
            //println!("Trace   | ボタンが押されたぜ☆（＾～＾） {:?}", pressed_pos);
            input_state.previous_point.x = input_state.pressed_point.x;
            input_state.previous_point.y = input_state.pressed_point.y;

            paint_tool.on_mouse_pressed(&settings, &input_state, k_image);
        }

        // TODO ⚡Mouse move
        // 📖 [MouseRelativeEvent](https://docs.piston.rs/piston_window/piston_window/trait.MouseRelativeEvent.html)
        if let Some(coord) = e.mouse_relative_args() {
            let dx = coord[0];
            let dy = coord[1];

            if input_state.is_mouse_pressed {
                input_state.moved_vector.x += dx;
                input_state.moved_vector.y += dy;
                //println!(
                //    "Trace   | マウス移動中☆（＾～＾） ({:?}, {:?}) ({:?}, {:?})",
                //    dx, dy, pressed_pos.x, pressed_pos.y
                //);
            }

            if paint_tool.on_mouse_moved(&settings, &input_state, k_image) {
                if input_state.is_mouse_pressed {
                    // 更新
                    input_state.previous_point.x += input_state.moved_vector.x;
                    input_state.previous_point.y += input_state.moved_vector.y;
                    input_state.moved_vector.x = 0.0;
                    input_state.moved_vector.y = 0.0;
                }
            }
        }

        // ⚡Mouse button released
        // 📖 [ReleaseEvent](https://docs.piston.rs/piston_window/piston_window/trait.ReleaseEvent.html)
        if let Some(_button) = e.release_args() {
            //println!("Trace   | ボタンを離したぜ☆（＾～＾）");
            paint_tool.on_mouse_released(&settings, &input_state, k_image);
            input_state.is_mouse_pressed = false;
            input_state.pressed_point.x = 0.0;
            input_state.pressed_point.y = 0.0;
            input_state.moved_vector.x = 0.0;
            input_state.moved_vector.y = 0.0;
            input_state.previous_point.x = 0.0;
            input_state.previous_point.y = 0.0;
        }

        // ⚡Window paint
        window.draw_2d(&e, |c, g, device| {
            clear([1.0; 4], g);

            /*
            // 画像を丸ごと再描画します。
            image(&texture, c.transform.zoom(settings.canvas_zoom), g);
            */

            // 点を１個描くぜ☆（＾～＾）データとしての保存は別のところでやってるぜ☆（＾～＾）
            // let sizing = KusaSize::diff(&cursor, &pressed_pos);

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
            let canvas_size = KusaSize {
                width: settings.image_width as f64 * settings.canvas_dot_width,
                height: settings.image_height as f64 * settings.canvas_dot_height,
            };
            Grid::draw(&settings, &canvas_size, &c, g);

            // TODO 座標を表示したいぜ☆（＾～＾）
            text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32)
                .draw(
                    &format!("xy({:.0}, {:.0})", k_mouse_cursor.x, k_mouse_cursor.y),
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
