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

    // Event loop.
    window.set_lazy(true);
    while let Some(e) = window.next() {
        // update
        if let Some(_button) = e.press_args() {
            println!("ボタンが押されたぜ☆（＾～＾）");
        }

        // draw
        window.draw_2d(&e, |c, g, _| {
            clear([1.0; 4], g);
            image(&texture, c.transform.zoom(2.0), g);
        });
    }
}
