use ggez::mint::Point2;

pub type Point = Point2<f32>;

#[derive(PartialEq, Clone, Copy)]
pub struct Size {
    pub width: f32,
    pub height: f32
}