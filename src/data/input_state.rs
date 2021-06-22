use crate::data::pointing::Pointing;

pub struct InputState {
    pub is_mouse_pressed: bool,
    pub pressed_coord: Pointing,
}
impl Default for InputState {
    fn default() -> Self {
        InputState {
            is_mouse_pressed: false,
            pressed_coord: Pointing::default(),
        }
    }
}
impl InputState {}
