pub use sdl2::rect::{ Point, Rect };

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Size {
    pub width: u32,
    pub height: u32
}

#[derive(Clone)]
pub struct AnchorPoint {
    pub x: f32,
    pub y: f32,
}

impl Default for AnchorPoint {

    fn default() -> Self {
        AnchorPoint { x: 0.5, y: 0.5 }
    }

}
