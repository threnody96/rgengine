pub use sdl2::rect::{ Point, Rect };

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Size {
    pub width: u32,
    pub height: u32
}
