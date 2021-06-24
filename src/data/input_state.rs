use crate::data::pointing::KusaPoint;

pub struct InputState {
    pub is_mouse_pressed: bool,
    /// マウスクリックしたスクリーン座標
    pub pressed_point: KusaPoint,
    /// 前回まで描画したスクリーン座標
    pub previous_point: KusaPoint,
    /// マウス移動したスクリーン座標の総和
    pub moved_vector: KusaPoint,
}
impl Default for InputState {
    fn default() -> Self {
        InputState {
            is_mouse_pressed: false,
            pressed_point: KusaPoint::default(),
            previous_point: KusaPoint::default(),
            moved_vector: KusaPoint::default(),
        }
    }
}
impl InputState {}
