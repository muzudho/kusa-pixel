use crate::data::pointing::KusaPoint;

pub struct InputState {
    pub is_mouse_pressed: bool,
    pub pressed_coord: KusaPoint,
}
impl Default for InputState {
    fn default() -> Self {
        InputState {
            is_mouse_pressed: false,
            pressed_coord: KusaPoint::default(),
        }
    }
}
impl InputState {}
