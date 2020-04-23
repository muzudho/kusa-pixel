use crate::res::image::{Dot, Frame};
use crate::res::logic::image_operation::*;
use crate::res::pointing::{Pointing, Sizing};
use crate::res::settings::Settings;
use crate::res::tool::Pen;
use piston_window::*;

pub fn show_window(mut settings: Settings, frame: &mut Frame) {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("piston: image", [300, 300])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let texture = create_texture(&settings.file, &mut window);
    let mut cursor = Pointing::default();
    let mut pressed_pos = cursor;

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    println!("{:?}", assets);
    let mut glyphs = window
        .load_font(assets.join("font/NotoSans-Medium.ttf"))
        .unwrap();

    let mut count: u64 = 0;
    // Event loop.
    window.set_lazy(true);
    while let Some(e) = window.next() {
        if count % 1000 == 999 {
            // ミリ秒の取り方が分からなかったぜ☆（＾～＾）
            // イベント・ループの中で　ファイル入出力するのは　クソだが　使い慣れてないんで仕方ないぜ☆（＾～＾）
            // 設定ファイルを監視するぜ☆（＾～＾）
            settings = Settings::load();
            println!(
                "Trace   | Load settings☆（＾～＾） tool=|{}|",
                settings.tool
            );
        }
        count += 1;
        // マウスカーソルの座標を補足するぜ☆（＾～＾）
        e.mouse_cursor(|pos| {
            cursor = Pointing::from_pos(pos, &settings);
        });

        // Pressed
        if let Some(_button) = e.press_args() {
            pressed_pos = cursor.clone();
            println!("Trace   | ボタンが押されたぜ☆（＾～＾） {:?}", pressed_pos);
        }

        if let Some(_button) = e.release_args() {
            println!("Trace   | ボタンを離したぜ☆（＾～＾）");
            let sizing = Sizing::diff(&cursor, &pressed_pos);

            // 線を引きます。
            Pen::draw(frame, &pressed_pos, &sizing);

            println!("Trace   | Click ({}, {}) 保存", &cursor.x, &cursor.y);
            write_frame(&frame, &settings.file);
        }

        // draw
        window.draw_2d(&e, |c, g, device| {
            clear([1.0; 4], g);

            // 線を引くのではなく、画像を丸ごと再描画します。
            image(&texture, c.transform.zoom(settings.zoom), g);

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

            // キャンバス幅
            let canvas_width = settings.width as f64 * settings.canvas_dot_width;
            let canvas_height = settings.height as f64 * settings.canvas_dot_height;

            // 各マスに色を打っていくぜ☆（＾～＾）
            // タテへ
            for row in 0..settings.height {
                // ヨコへ
                for col in 0..settings.width {
                    let dot = frame.get_dot(col, row);
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

            // タテ線
            for col in 0..(settings.width + 1) {
                line(
                    settings.canvas_grid_color,
                    settings.canvas_grid_thickness, // radius
                    [
                        col as f64 * settings.canvas_dot_width + settings.canvas_margin_x,
                        settings.canvas_margin_y,
                        col as f64 * settings.canvas_dot_width + settings.canvas_margin_x,
                        settings.canvas_margin_y + canvas_height,
                    ],
                    c.transform,
                    g,
                );
            }

            // ヨコ線
            for row in 0..(settings.height + 1) {
                line(
                    settings.canvas_grid_color,
                    settings.canvas_grid_thickness, // radius
                    [
                        settings.canvas_margin_x,
                        row as f64 * settings.canvas_dot_height + settings.canvas_margin_y,
                        settings.canvas_margin_x + canvas_width,
                        row as f64 * settings.canvas_dot_height + settings.canvas_margin_y,
                    ],
                    c.transform,
                    g,
                );
            }

            // TODO 座標を表示したいぜ☆（＾～＾）
            text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32)
                .draw(
                    &format!("cell({}, {})", cursor.x, cursor.y),
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
