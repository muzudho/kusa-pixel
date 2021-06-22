pub mod pen;

use crate::data::input_state::InputState;
use crate::piston_wrapper::kusa_image::KusaImage;
use crate::settings::Settings;
use piston_window::*;

pub trait PaintTool {
    fn on_mouse_pressed(
        &self,
        settings: &Settings,
        input_state: &InputState,
        k_image: &mut KusaImage,
    );
    fn on_mouse_released(
        &self,
        settings: &Settings,
        input_state: &InputState,
        k_image: &mut KusaImage,
    );
    fn on_mouse_moved(
        &self,
        settings: &Settings,
        input_state: &InputState,
        k_image: &mut KusaImage,
    );
}

// 画像上の座標
pub fn coord_on_image(x: f64, y: f64, settings: &Settings) -> Option<(i32, i32)> {
    let xx = (x - settings.canvas_margin_left) / settings.canvas_dot_width;
    let yy = (y - settings.canvas_margin_top) / settings.canvas_dot_height;

    if 0.0 <= xx
        && xx < settings.image_width as f64
        && 0.0 <= yy
        && yy < settings.image_height as f64
    {
        return Some((xx as i32, yy as i32));
    } else {
        return None;
    }
}

pub struct PaintOperation {}
impl PaintOperation {
    /// 各マスに色を打っていくぜ☆（＾～＾）
    pub fn draw_image(settings: &Settings, k_image: &KusaImage, c: &Context, g: &mut G2d) {
        // タテへ
        for row in 0..settings.image_height {
            // ヨコへ
            for col in 0..settings.image_width {
                let k_color = k_image.get_pixel(col, row);
                let x = col as f64 * settings.canvas_dot_width + settings.canvas_margin_left;
                let y = row as f64 * settings.canvas_dot_height + settings.canvas_margin_top;
                rectangle(
                    k_color.to_rgba_rate_array(),
                    [x, y, settings.canvas_dot_width, settings.canvas_dot_height],
                    c.transform,
                    g,
                );
            }
        }
    }
}
