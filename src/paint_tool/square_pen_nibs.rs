use crate::data::input_state::InputState;
use crate::data::pointing::KusaPoint;
use crate::paint_tool::screen_to_image_f;
use crate::paint_tool::PaintTool;
use crate::piston_wrapper::kusa_image::KusaImage;
use crate::settings::Settings;
use crate::write_k_image;

/// 正方形のペン先
pub struct SquarePenNibs {}
impl SquarePenNibs {
    // 点を置くぜ（＾～＾）
    fn put_pixel(settings: &Settings, k_image: &mut KusaImage, sc_center: &KusaPoint) {
        if let Some(center) = screen_to_image_f(settings, sc_center) {
            // 半径
            let radius = settings.paint_thickness / 2.0;
            let left = (center.x - radius) as i16;
            let right = (center.x + radius) as i16;
            let top = (center.y - radius) as i16;
            let bottom = (center.y + radius) as i16;

            for y in top..bottom {
                for x in left..right {
                    // 点を１個打って画像として保存するぜ☆（＾～＾）画面への描画は別のところでやってるぜ☆（＾～＾）
                    k_image.set_pixel(x as u32, y as u32, &settings.paint_color);
                }
            }
        }
    }
}
