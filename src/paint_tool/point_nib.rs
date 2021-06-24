use crate::data::pointing::KusaPoint;
use crate::paint_tool::Nib;
use crate::piston_wrapper::kusa_image::KusaImage;
use crate::settings::Settings;

/// 点のペン先（1 pixel の点です）
pub struct PointNib {}
impl Nib for PointNib {
    // 点を置くぜ（＾～＾）
    fn put_pixel(&self, settings: &Settings, k_image: &mut KusaImage, center: &KusaPoint) {
        // 点を１個打って画像として保存するぜ☆（＾～＾）画面への描画は別のところでやってるぜ☆（＾～＾）
        if 0.0 <= center.x
            && center.x < settings.image_width as f64
            && 0.0 <= center.y
            && center.y < settings.image_height as f64
        {
            k_image.set_pixel(center.x as u32, center.y as u32, &settings.paint_color);
        }
    }
}
