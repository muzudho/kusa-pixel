use crate::data::pointing::KusaSize;
use crate::settings::Settings;
use piston_window::*;

pub struct Grid {}
impl Grid {
    pub fn draw(settings: &Settings, canvas_size: &KusaSize, c: &Context, g: &mut G2d) {
        // タテ線
        for col in 0..(settings.image_width + 1) {
            line(
                settings.canvas_grid_color,
                settings.canvas_grid_thickness, // radius
                [
                    col as f64 * settings.canvas_dot_width + settings.canvas_margin_left,
                    settings.canvas_margin_top,
                    col as f64 * settings.canvas_dot_width + settings.canvas_margin_left,
                    settings.canvas_margin_top + canvas_size.height,
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
                    settings.canvas_margin_left,
                    row as f64 * settings.canvas_dot_height + settings.canvas_margin_top,
                    settings.canvas_margin_left + canvas_size.width,
                    row as f64 * settings.canvas_dot_height + settings.canvas_margin_top,
                ],
                c.transform,
                g,
            );
        }
    }
}
