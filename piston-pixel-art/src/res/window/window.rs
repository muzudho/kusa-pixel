extern crate find_folder;
extern crate piston_window;

use piston_window::*;

pub fn show_window(png_path: &str) {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("piston: image", [300, 300])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let rust_logo = assets.join(png_path);
    let rust_logo: G2dTexture = Texture::from_path(
        &mut window.create_texture_context(),
        &rust_logo,
        Flip::None,
        &TextureSettings::new(),
    )
    .unwrap();

    // Event loop.
    window.set_lazy(true);
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([1.0; 4], g);
            image(&rust_logo, c.transform, g);
        });
    }
}
