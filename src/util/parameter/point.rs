use std::ops::{ Deref };

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Point {
    point: sdl2::rect::Point
}

impl Point {

    pub fn new(x: i32, y: i32) -> Self {
        Self {
            point: sdl2::rect::Point::new(x, y)
        }
    }

}

impl Deref for Point {

    type Target = sdl2::rect::Point;

    fn deref(&self) -> &Self::Target {
        &self.point
    }

}

impl Into<sdl2::rect::Point> for Point {

    fn into(self) -> sdl2::rect::Point {
        self.point
    }

}

impl From<sdl2::rect::Point> for Point {

    fn from(f: sdl2::rect::Point) -> Self {
        Self::new(f.x(), f.y())
    }

}

impl From<&sdl2::rect::Point> for Point {

    fn from(f: &sdl2::rect::Point) -> Self {
        Self::from(f.clone())
    }

}

impl From<&Point> for Point {

    fn from(f: &Point) -> Point {
        f.clone()
    }

}

