use crate::res::logic::image_operation::*;
use crate::res::settings::Settings;
// use gfx::*;
// use gfx_device_gl::*;
use piston_window::*;

pub fn show_window(settings: &Settings) {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("piston: image", [300, 300])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let texture = create_texture(&settings.file, &mut window);
    let mut cursor = [0.0, 0.0];
    let mut col = 0;
    let mut row = 0;

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    println!("{:?}", assets);
    let mut glyphs = window
        .load_font(assets.join("font/NotoSans-Medium.ttf"))
        .unwrap();

    // Event loop.
    window.set_lazy(true);
    while let Some(e) = window.next() {
        // マウスカーソルの座標を補足するぜ☆（＾～＾）
        e.mouse_cursor(|pos| {
            cursor = pos;
            col = ((cursor[0] - settings.canvas_margin_x) / settings.canvas_dot_width) as i32;
            row = ((cursor[1] - settings.canvas_margin_y) / settings.canvas_dot_height) as i32;
        });

        // update
        if let Some(_button) = e.press_args() {
            println!("ボタンが押されたぜ☆（＾～＾）");
            println!("Click ({}, {})", cursor[0], cursor[1]);
        }

        // draw
        window.draw_2d(&e, |c, g, device| {
            clear([1.0; 4], g);
            image(&texture, c.transform.zoom(2.0), g);

            rectangle(
                [1.0, 0.0, 0.0, 1.0], // red
                [cursor[0], cursor[1], 100.0, 100.0],
                c.transform,
                g,
            );

            // TODO 座標を表示したいぜ☆（＾～＾）
            text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32)
                .draw(
                    &format!("pos({}, {})", col, row),
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(10.0, 30.0), // y位置を揃えるのはむずかしいぜ☆（＾～＾）
                    g,
                )
                .unwrap();

            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);

            // キャンバス幅
            let canvas_width = settings.width as f64 * settings.canvas_dot_width;
            let canvas_height = settings.height as f64 * settings.canvas_dot_height;

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
        });
    }
}
