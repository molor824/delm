use raylib::{color::Color, math::Vector2};

use crate::renderer::{Corner, Renderer};

pub trait Widget<Msg> {
    type State;
    fn init_state(&self) -> Self::State;
    fn update_state(&self, state: &mut Self::State);
    fn render(&self, state: &Self::State, renderer: &mut Renderer);
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Length {
    pub absolute: f32,
    pub scale: f32,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct UiRect {
    pub x: Length,
    pub y: Length,
    pub width: Length,
    pub height: Length,
}

pub struct RectFrame {
    pub rect: UiRect,
    pub max_size: Vector2,
    pub min_size: Vector2,
    pub color: Color,
    pub rounding: Corner,
}
impl<Msg> Widget<Msg> for RectFrame {
    type State = ();
    fn init_state(&self) -> Self::State {
        ()
    }
    fn update_state(&self, _state: &mut Self::State) {}
    fn render(&self, _state: &Self::State, _renderer: &mut Renderer) {
        todo!()
    }
}
