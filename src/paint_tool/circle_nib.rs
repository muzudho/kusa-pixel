use crate::data::pointing::KusaPoint;
use crate::paint_tool::Nib;
use crate::piston_wrapper::kusa_image::KusaImage;
use crate::settings::Settings;

/// 円形のペン先
pub struct CircleNib {}
impl Nib for CircleNib {
    // 点を置くぜ（＾～＾）
    fn put_pixel(&self, settings: &Settings, k_image: &mut KusaImage, center: &KusaPoint) {
        // 半径
        let radius = settings.paint_thickness / 2.0;

        // それぞれ、1px 広めにとる
        let left = {
            // 四捨五入
            let mut left = (center.x - radius).round() as i16 - 1;
            if left < 0 {
                left = 0;
            }
            left
        };

        let right = {
            let mut right = (center.x + radius).round() as i16 + 1;
            if settings.image_width as i16 <= right {
                right = (settings.image_width - 1) as i16;
            }
            right as i16
        };

        let top = {
            let mut top = (center.y - radius).round() as i16 - 1;
            if top < 0 {
                top = 0;
            }
            top
        };

        let bottom = {
            let mut bottom = (center.y + radius).round() as i16 + 1;
            if settings.image_height as i16 <= bottom {
                bottom = (settings.image_width - 1) as i16;
            }
            bottom as i16
        };
        // println!(
        //     "Trace   | left={} right={} top={} bottom={}",
        //     left, right, top, bottom
        // );

        for y in top..bottom {
            let cell_center_y = y as f64; //  + 0.5
            let height = (center.y - cell_center_y).abs();
            for x in left..right {
                let cell_center_x = x as f64; //  + 0.5
                let width = (center.x - cell_center_x).abs();
                let radius2 = (width * width + height * height).sqrt();

                // 円の中なら塗る（＾～＾）
                if radius2 < radius {
                    // 点を１個打って画像として保存するぜ☆（＾～＾）画面への描画は別のところでやってるぜ☆（＾～＾）
                    k_image.set_pixel(x as u32, y as u32, &settings.paint_color);
                    /*
                    } else {
                        k_image.set_pixel(
                            x as u32,
                            y as u32,
                            &KusaColor {
                                r: 0,
                                g: 0,
                                b: 0,
                                a: 255,
                            },
                        );
                        */
                }
            }
        }
    }
}
