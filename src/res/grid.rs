use crate::res::pointing::Sizing;
use crate::res::settings::Settings;
use piston_window::*;

pub struct Grid {}
impl Grid {
    pub fn draw(settings: &Settings, canvas_size: &Sizing, c: &Context, g: &mut G2d) {
        // タテ線
        for col in 0..(settings.image_width + 1) {
            line(
                settings.canvas_grid_color,
                settings.canvas_grid_thickness, // radius
                [
                    col as f64 * settings.canvas_dot_width + settings.canvas_margin_x,
                    settings.canvas_margin_y,
                    col as f64 * settings.canvas_dot_width + settings.canvas_margin_x,
                    settings.canvas_margin_y + canvas_size.height,
                ],
                c.transform,
                g,
            );
        }

        // ヨコ線
        for row in 0..(settings.image_height + 1) {
            line(
                settings.canvas_grid_color,
                settings.canvas_grid_thickness, // radius
                [
                    settings.canvas_margin_x,
                    row as f64 * settings.canvas_dot_height + settings.canvas_margin_y,
                    settings.canvas_margin_x + canvas_size.width,
                    row as f64 * settings.canvas_dot_height + settings.canvas_margin_y,
                ],
                c.transform,
                g,
            );
        }
    }
}
