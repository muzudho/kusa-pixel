pub mod circle_nib;
pub mod pen;
pub mod point_nib;
pub mod square_nib;

use crate::data::input_state::InputState;
use crate::data::pointing::KusaCell;
use crate::data::pointing::KusaPoint;
use crate::piston_wrapper::kusa_image::KusaImage;
use crate::settings::Settings;
use piston_window::*;

pub trait PaintTool {
    fn on_mouse_pressed(
        &self,
        settings: &Settings,
        nib: &dyn Nib,
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
        nib: &dyn Nib,
        input_state: &InputState,
        k_image: &mut KusaImage,
    ) -> bool;
}

pub trait Nib {
    fn put_pixel(&self, settings: &Settings, k_image: &mut KusaImage, center: &KusaPoint);
}

/// # Arguments
///
/// * `sc_coord` - スクリーン座標
///
/// # Returns
///
/// 画像上の座標
pub fn screen_to_table(settings: &Settings, sc_coord: &KusaPoint) -> KusaCell {
    // 画像上の座標
    let im_x = (sc_coord.x - settings.canvas_margin_left) / settings.canvas_cell_size;
    let im_y = (sc_coord.y - settings.canvas_margin_top) / settings.canvas_cell_size;

    return KusaCell {
        x: im_x as i32,
        y: im_y as i32,
    };
}

/// # Arguments
///
/// * `sc_coord` - スクリーン座標
///
/// # Returns
///
/// 画像上の座標
pub fn screen_to_table_f(settings: &Settings, sc_coord: &KusaPoint) -> KusaPoint {
    // 画像上の座標
    let im_x = (sc_coord.x - settings.canvas_margin_left) / settings.canvas_cell_size;
    let im_y = (sc_coord.y - settings.canvas_margin_top) / settings.canvas_cell_size;

    return KusaPoint { x: im_x, y: im_y };
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
                let x = col as f64 * settings.canvas_cell_size + settings.canvas_margin_left;
                let y = row as f64 * settings.canvas_cell_size + settings.canvas_margin_top;
                rectangle(
                    k_color.to_rgba_rate_array(),
                    [x, y, settings.canvas_cell_size, settings.canvas_cell_size],
                    c.transform,
                    g,
                );
            }
        }
    }
}
