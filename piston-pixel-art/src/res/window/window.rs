extern crate find_folder;
extern crate piston_window;

use crate::res::logic::image_operation::*;
use piston_window::*;

pub fn show_window(png_path: &str) {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("piston: image", [300, 300])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let texture = create_texture(png_path, &mut window);
    let mut cursor = [0.0, 0.0];

    // Event loop.
    window.set_lazy(true);
    while let Some(e) = window.next() {
        // マウスカーソルの座標を補足するぜ☆（＾～＾）
        e.mouse_cursor(|pos| {
            cursor = pos;
            // println!("Mouse moved '{} {}'", pos[0], pos[1]);
        });

        // update
        if let Some(_button) = e.press_args() {
            println!("ボタンが押されたぜ☆（＾～＾）");
            println!("Click ({}, {})", cursor[0], cursor[1]);
        }

        // draw
        window.draw_2d(&e, |c, g, _| {
            clear([1.0; 4], g);
            image(&texture, c.transform.zoom(2.0), g);

            rectangle(
                [1.0, 0.0, 0.0, 1.0], // red
                [cursor[0], cursor[1], 100.0, 100.0],
                c.transform,
                g,
            );
        });
    }
}
