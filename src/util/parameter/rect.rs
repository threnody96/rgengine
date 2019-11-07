use std::ops::Deref;
use ::util::parameter::{ Point };

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Rect {
    rect: sdl2::rect::Rect
}

impl Rect {

    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            rect: sdl2::rect::Rect::new(x, y, width, height)
        }
    }

    pub fn from_center<P>(center: P, width: u32, height: u32) -> Self
    where P: Into<Point>
    {
        Self {
            rect: sdl2::rect::Rect::from_center(center.into(), width, height)
        }
    }

}

impl Deref for Rect {

    type Target = sdl2::rect::Rect;

    fn deref(&self) -> &Self::Target {
        &self.rect
    }

}

impl Into<sdl2::rect::Rect> for Rect {

    fn into(self) -> sdl2::rect::Rect {
        self.rect
    }

}

impl From<&Rect> for Rect {

    fn from(f: &Rect) -> Rect {
        f.clone()
    }

}

